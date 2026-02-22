use error_stack::{Report, ResultExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use secrecy::{Secret, ExposeSecret};
use common::constants;

fn verify_access_token(
    jwt_token: Secret<String>,
    secret: Secret<String>
) -> Result<types::Claims, Report<types::AuthErrorTypes>> {

    let unmasked_secret = secret.expose_secret();
    let token = jwt_token.expose_secret();

    let mut validation = Validation::default();
    validation.set_issuer(&[constants::ISSUER]);
    validation.set_audience(&[constants::AUDIENCE]);

    let decoded = decode::<types::Claims>(
        token,
        &DecodingKey::from_secret(unmasked_secret.as_bytes()),
        &validation
    )
    .into_report()
    .change_context(types::AuthErrorTypes::InvalidToken)?;

    Ok(decoded.claims)
}