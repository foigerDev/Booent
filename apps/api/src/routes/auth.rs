
use axum::{extract::State, http::StatusCode, Json, response::IntoResponse};
use crate::{app_state::AppState, api_models::auth as api_models_auth};
use std::sync::Arc;
use auth::google_auth::verify_google_login;
use common::errors::ApiError;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<api_models_auth::LoginRequest>,
) -> Result<(StatusCode, &'static str), ApiError> {

    let claims = verify_google_login(
        payload.into(),
        state.config.google_config.client_id.clone()
    )
    .await
    .map_err(ApiError::Auth)?;   

    Ok((StatusCode::OK, "OK"))
}
