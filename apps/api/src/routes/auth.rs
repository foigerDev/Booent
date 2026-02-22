use crate::{api_models::auth as api_models_auth, app_state::AppState};
use auth::google_auth::verify_google_login;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use common::errors::ApiError;
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<api_models_auth::LoginRequest>,
) -> Result<(StatusCode, &'static str), ApiError> {
    let claims = verify_google_login(payload.into(), state.config.google_config.client_id.clone())
        .await
        .map_err(ApiError::Auth)?;

    Ok((StatusCode::OK, "OK"))
}
