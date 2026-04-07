use crate::{api_models::auth as api_models_auth, app_state::AppState, utils};
use auth::user_management::verifiers::verify_google_login;
use axum::extract::State;
use axum::{
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use common::consts;
use common::errors::ApiError;
use secrecy::ExposeSecret;
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<api_models_auth::LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    utils::validate_api_key(&headers, state.config.admin_api_key.expose_secret())?;

    let claims = verify_google_login(payload.into(), state.config.google_config.client_id.clone())
        .await
        .map_err(ApiError::Auth)?;

    let tokens =
        auth::user_management::services::user_login(&state.db, state.config.clone(), claims)
            .await
            .map_err(ApiError::Auth)?;

    let cookie = utils::build_refresh_token_cookie(tokens.refresh_token.expose_secret());
    let response_body = api_models_auth::LoginResponse::from(tokens);

    let mut response = Json(response_body).into_response();
    response
        .headers_mut()
        .append(SET_COOKIE, cookie.to_string().parse().unwrap());

    *response.status_mut() = StatusCode::OK;

    Ok(response)
}

pub async fn signup(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<api_models_auth::LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    utils::validate_api_key(&headers, state.config.admin_api_key.expose_secret())?;
    let claims = verify_google_login(payload.into(), state.config.google_config.client_id.clone())
        .await
        .map_err(ApiError::Auth)?;
    let tokens =
        auth::user_management::services::user_sign_up(&state.db, state.config.clone(), claims)
            .await
            .map_err(ApiError::Auth)?;

    let cookie = utils::build_refresh_token_cookie(tokens.refresh_token.expose_secret());
    let response_body = api_models_auth::LoginResponse::from(tokens);

    let mut response = Json(response_body).into_response();
    response
        .headers_mut()
        .append(SET_COOKIE, cookie.to_string().parse().unwrap());

    *response.status_mut() = StatusCode::OK;

    Ok(response)
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    let refresh_token = cookies
        .get(consts::REFRESH_TOKEN_COOKIE_NAME)
        .map(|cookie: &cookie::Cookie| cookie.value().to_string())
        .ok_or(ApiError::Auth(
            common::errors::AuthErrorTypes::RefreshTokenInvalid.into(),
        ))?;

    let tokens = auth::user_management::services::refresh_tokens(&state.db, &refresh_token, state.config.jwt_secret.clone())
        .await
        .map_err(ApiError::Auth)?;

    let cookie = utils::build_refresh_token_cookie(tokens.refresh_token.expose_secret());
    let response_body = api_models_auth::LoginResponse::from(tokens);

    let mut response = Json(response_body).into_response();
    response
        .headers_mut()
        .append(SET_COOKIE, cookie.to_string().parse().unwrap());

    *response.status_mut() = StatusCode::OK;

    Ok(response)
}
