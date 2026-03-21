use axum::http::HeaderMap;
use common::{
    consts::{API_KEY_HEADER, MAX_REFRESH_TOKEN_VALIDITY_IN_MINUTES},
    errors::{ApiError, AuthErrorTypes},
};
use cookie::{time::Duration, Cookie, SameSite};

const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";
const COOKIE_DOMAIN: &str = "yourdomain.com"; // Set from config in production

pub fn build_refresh_token_cookie(token: &str) -> Cookie<'static> {
    let max_age = Duration::minutes(MAX_REFRESH_TOKEN_VALIDITY_IN_MINUTES as i64);

    Cookie::build((REFRESH_TOKEN_COOKIE_NAME, token.to_string()))
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(max_age)
        .domain(COOKIE_DOMAIN)
        .build()
}

pub fn validate_api_key(headers: &HeaderMap, expected: &str) -> Result<(), ApiError> {
    let api_key = headers
        .get(API_KEY_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::Auth(AuthErrorTypes::ApiAuthorizationFailed.into()))?;

    if api_key != expected {
        return Err(ApiError::Auth(
            AuthErrorTypes::ApiAuthorizationFailed.into(),
        ));
    }

    Ok(())
}
