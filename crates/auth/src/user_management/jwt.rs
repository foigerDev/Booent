use common::common_enums::Role;
use common::domain_models::{auth, common as common_domain_models};
use common::{consts, errors};
use error_stack::{Report, ResultExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use secrecy::{ExposeSecret, Secret};

pub fn verify_access_token(
    jwt_token: String,
    secret: Secret<String>,
) -> Result<common_domain_models::RequestContext, Report<errors::AuthErrorTypes>> {
    let unmasked_secret = secret.expose_secret();

    let mut validation = Validation::default();
    validation.set_issuer(&[consts::ISSUER]);
    validation.set_audience(&[consts::AUDIENCE]);

    let decoded = decode::<auth::AccessToken>(
        jwt_token,
        &DecodingKey::from_secret(unmasked_secret.as_bytes()),
        &validation,
    )
    .change_context(errors::AuthErrorTypes::InvalidToken)?;

    Ok(common_domain_models::RequestContext {
        user_id: decoded.claims.sub,
    })
}

pub fn generate_access_tokens(
    user_id: String,
    session_id: String,
    jwt_secret: Secret<String>,
) -> Result<Secret<String>, Report<errors::AuthErrorTypes>> {
    let now = time::OffsetDateTime::now_utc();

    let access_expiry = now + time::Duration::minutes(consts::MAX_ACCESS_TOKEN_VALIDITY_IN_MINUTES);

    let claims = auth::AccessToken {
        iss: consts::ISSUER.to_string(),
        sub: user_id,
        sid: session_id,
        aud: consts::AUDIENCE.to_string(),
        exp: access_expiry,
        iat: now,
        nbf: now,
        role: Role::Admin,
    };

    let access_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.expose_secret().as_bytes()),
    )
    .attach_printable("Failed to generate access token")
    .change_context(errors::AuthErrorTypes::InternalServerError)?;

    Ok(Secret::new(access_token))
}
