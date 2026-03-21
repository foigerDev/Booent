use common::{consts, domain_models::auth, errors};
use error_stack::{Report, ResultExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use secrecy::{ExposeSecret, Secret};

fn _verify_access_token(
    jwt_token: Secret<String>,
    secret: Secret<String>,
) -> Result<auth::GoogleUserClaims, Report<errors::AuthErrorTypes>> {
    let unmasked_secret = secret.expose_secret();
    let token: &String = jwt_token.expose_secret();

    let mut validation = Validation::default();
    validation.set_issuer(&[consts::ISSUER]);
    validation.set_audience(&[consts::AUDIENCE]);

    let decoded = decode::<auth::GoogleUserClaims>(
        token,
        &DecodingKey::from_secret(unmasked_secret.as_bytes()),
        &validation,
    )
    .change_context(errors::AuthErrorTypes::InvalidToken)?;

    Ok(decoded.claims)
}

pub fn generate_access_tokens(
    user_id: String,
    session_id: String,
    jwt_secret: Secret<String>,
) -> Result<Secret<String>, error_stack::Report<errors::AuthErrorTypes>> {
    let now = time::OffsetDateTime::now_utc();

    let access_expiry = now + time::Duration::minutes(consts::MAX_ACCESS_TOKEN_VALIDITY_IN_MINUTES);

    let claims = common::domain_models::auth::AccessToken {
        iss: consts::ISSUER.to_string(),
        sub: user_id,
        sid: session_id,
        aud: consts::AUDIENCE.to_string(),
        exp: access_expiry,
        iat: now,
        nbf: now,
        role: common::common_enums::Role::Admin,
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
