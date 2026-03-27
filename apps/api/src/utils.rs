use axum::http::HeaderMap;
use common::{
    consts,
    errors::{ApiError, AuthErrorTypes},
};
use cookie::{time::Duration, Cookie, SameSite};
use error_stack::ResultExt;


pub fn build_refresh_token_cookie(token: &str) -> Cookie<'static> {
    let max_age = Duration::minutes(consts::MAX_REFRESH_TOKEN_VALIDITY_IN_MINUTES);

    Cookie::build((consts::REFRESH_TOKEN_COOKIE_NAME, token.to_string()))
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(max_age)
        // .domain(consts::COOKIE_DOMAIN)
        .build()
}

pub fn validate_api_key(headers: &HeaderMap, expected: &str) -> Result<(), ApiError> {
    let api_key = headers.get(consts::API_KEY_HEADER)
    .ok_or(ApiError::Auth(AuthErrorTypes::ApiAuthorizationFailed.into()))?
    .to_str()
    .map_err(|_| ApiError::Auth(AuthErrorTypes::ApiAuthorizationFailed.into()))?;

    if api_key != expected {
        return Err(ApiError::Auth(
            AuthErrorTypes::ApiAuthorizationFailed.into(),
        ));
    }

    Ok(())
}


pub trait HeaderMapExt {
    fn extract_access_token_from_header(&self) -> Result<String, ApiError>;
}

impl HeaderMapExt for HeaderMap {
    fn extract_access_token_from_header(&self) -> Result<String, ApiError> {
        let bearer_token =   self.get(consts::AUTHORIZATION)
        .ok_or(ApiError::Auth(AuthErrorTypes::ApiAuthorizationFailed.into()))?
        .to_str()
        .map_err(|_| ApiError::Auth(AuthErrorTypes::ApiAuthorizationFailed.into()))?;

        let access_token = bearer_token.strip_prefix("Bearer ").ok_or(ApiError::Auth(
            AuthErrorTypes::ApiAuthorizationFailed.into(),
        ))?;

        Ok(access_token.to_string())
    }
    }
