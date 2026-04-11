// use error_stack::{Report, ResultExt};
// use std::sync::Arc;
use common::{domain_models::auth, errors::AuthErrorTypes};
use error_stack::ResultExt;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use secrecy::ExposeSecret;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

const GOOGLE_CERT_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v3/certs";
const GOOGLE_ISSUER_ENDPOINT: &str = "https://accounts.google.com";
const GOOGLE_ISSUER_DOMAIN: &str = "accounts.google.com";
const JWKS_CACHE_TTL_SECONDS: i64 = 3600;

lazy_static::lazy_static! {
    static ref JWKS_CACHE: Arc<RwLock<Option<(Jwks, time::OffsetDateTime)>>> = Arc::new(RwLock::new(None));
}

async fn fetch_google_jwks_internal() -> Result<Jwks, error_stack::Report<AuthErrorTypes>> {
    let jwks: Jwks = reqwest::get(GOOGLE_CERT_ENDPOINT)
        .await
        .change_context(AuthErrorTypes::GoogleKeysFetchFailed)?
        .json()
        .await
        .change_context(AuthErrorTypes::InternalServerError)?;

    Ok(jwks)
}

pub async fn fetch_google_jwks() -> Result<Jwks, error_stack::Report<AuthErrorTypes>> {
    let cache = JWKS_CACHE.read().await;
    
    if let Some((jwks, cached_at)) = cache.as_ref() {
        let ttl = time::Duration::seconds(JWKS_CACHE_TTL_SECONDS);
        if *cached_at + ttl > time::OffsetDateTime::now_utc() {
            return Ok(jwks.clone());
        }
    }
    drop(cache);

    let jwks = fetch_google_jwks_internal().await?;
    
    let mut cache = JWKS_CACHE.write().await;
    *cache = Some((jwks.clone(), time::OffsetDateTime::now_utc()));
    
    Ok(jwks)
}

pub async fn verify_google_login(
    request: auth::GoogleLoginRequest,
    client_id: String,
) -> Result<auth::GoogleUserClaims, error_stack::Report<AuthErrorTypes>> {
    let jwks = fetch_google_jwks().await?;

    let header = decode_header(request.id_token.clone())
        .change_context(AuthErrorTypes::InvalidGoogleToken)?;
    let kid = header.kid.ok_or(AuthErrorTypes::InternalServerError)?;

    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.kid == kid)
        .ok_or(AuthErrorTypes::GoogleJWKNotFound)?;

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .change_context(AuthErrorTypes::InternalServerError)?;
    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_audience(&[client_id]);
    validation.set_issuer(&[GOOGLE_ISSUER_ENDPOINT, GOOGLE_ISSUER_DOMAIN]);

    let token_data =
        decode::<auth::GoogleUserClaims>(request.id_token.clone(), &decoding_key, &validation)
            .change_context(AuthErrorTypes::InvalidToken)?;

    let claims = token_data.claims;

    if claims.email_verified != Some(true) {
        return Err(AuthErrorTypes::GoogleEmailNotVerified.into());
    };

    log::info!(
        "user_verified: {}",
        claims
            .email
            .clone()
            .ok_or(AuthErrorTypes::DataNotFound {
                field_name: "Email".to_string()
            })?
            .expose_secret()
    );

    Ok(claims)
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwk {
    pub kid: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}
