// use error_stack::{Report, ResultExt};
// use std::sync::Arc;
use common::{domain_models::auth, errors::AuthErrorTypes};
use error_stack::ResultExt;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use secrecy::ExposeSecret;
use serde::Deserialize;

const GOOGLE_CERT_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v3/certs";
const GOOGLE_ISSUER_ENDPOINT: &str = "https://accounts.google.com";
const GOOGLE_ISSUER_DOMAIN: &str = "accounts.google.com";

pub async fn fetch_google_jwks() -> Result<Jwks, error_stack::Report<AuthErrorTypes>> {
    let jwks: Jwks = reqwest::get(GOOGLE_CERT_ENDPOINT)
        .await
        .change_context(AuthErrorTypes::GoogleKeysFetchFailed)?
        .json()
        .await
        .change_context(AuthErrorTypes::InternalServerError)?;

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

#[derive(Debug, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}
