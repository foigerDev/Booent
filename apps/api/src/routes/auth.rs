use crate::{api_models::auth as api_models_auth, app_state::AppState};
use auth::google_auth::verify_google_login;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use common::errors::{ApiError, AuthErrorTypes};
use secrecy::ExposeSecret;
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<api_models_auth::LoginRequest>,
) -> Result<(StatusCode, &'static str), ApiError> {

    // Move to Middleware
    let api_key = headers
        .get("api_key")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::Auth(
            AuthErrorTypes::ApiAuthorizationFailed.into(),
        ))?;

    if api_key != state.config.admin_api_key.expose_secret() {
        return Err(ApiError::Auth(
            AuthErrorTypes::ApiAuthorizationFailed.into(),
        ));
    }

    let claims = verify_google_login(payload.into(), state.config.google_config.client_id.clone())
        .await
        .map_err(ApiError::Auth)?;

    

    Ok((StatusCode::OK, "OK"))
}
