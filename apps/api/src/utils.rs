use axum::http::HeaderMap;
use common::{
    consts,
    db::hotels_interface::HotelRepository,
    db::users_interface::UserRepository,
    errors::{ApiError, AuthErrorTypes, HotelErrorTypes},
};
use cookie::{time::Duration, Cookie, SameSite};
use error_stack::ResultExt;
use subtle::ConstantTimeEq;
use uuid::Uuid;

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
    let api_key = headers
        .get(consts::API_KEY_HEADER)
        .ok_or(ApiError::Auth(
            AuthErrorTypes::ApiAuthorizationFailed.into(),
        ))?
        .to_str()
        .map_err(|_| ApiError::Auth(AuthErrorTypes::ApiAuthorizationFailed.into()))?;

    if api_key.as_bytes().ct_eq(expected.as_bytes()).into() {
        Ok(())
    } else {
        Err(ApiError::Auth(
            AuthErrorTypes::ApiAuthorizationFailed.into(),
        ))
    }
}

pub trait HeaderMapExt {
    fn extract_access_token_from_header(&self) -> Result<String, ApiError>;
}

impl HeaderMapExt for HeaderMap {
    fn extract_access_token_from_header(&self) -> Result<String, ApiError> {
        let bearer_token = self
            .get(consts::AUTHORIZATION)
            .ok_or(ApiError::Auth(
                AuthErrorTypes::ApiAuthorizationFailed.into(),
            ))?
            .to_str()
            .map_err(|_| ApiError::Auth(AuthErrorTypes::ApiAuthorizationFailed.into()))?;

        let access_token = bearer_token.strip_prefix("Bearer ").ok_or(ApiError::Auth(
            AuthErrorTypes::ApiAuthorizationFailed.into(),
        ))?;

        Ok(access_token.to_string())
    }
}

pub async fn validate_user_owns_hotel(
    db: &sqlx::PgPool,
    admin_api_key: &secrecy::Secret<String>,
    user_id: &str,
    hotel_id: Uuid,
) -> Result<(), ApiError> {
    let _ = db
        .find_user_by_user_id(user_id, admin_api_key)
        .await
        .change_context(AuthErrorTypes::UserNotFound)
        .map_err(ApiError::Auth)?;

    let user_owns_hotel = db
        .check_user_owns_hotel(user_id, hotel_id)
        .await
        .map_err(ApiError::Hotel)?;

    if !user_owns_hotel {
        return Err(ApiError::Hotel(
            error_stack::Report::new(HotelErrorTypes::UnauthorizedHotelAccess),
        ));
    }

    Ok(())
}
