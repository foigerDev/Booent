use crate::{api_models::hotels as api_models_hotels, app_state::AppState, utils};
use auth::user_management::verifiers::verify_google_login;
use axum::{
    extract::State,
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use common::errors::ApiError;
use secrecy::ExposeSecret;
use std::sync::Arc;
use crate::middleware;

pub async fn hotel_create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<api_models_hotels::HotelCreateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let request_context = middleware::middleware(state.config.clone(), HeaderMap::new())?;
    let hotel_response = hotels::services::create_hotel(&state.db, payload.into()).await.map_err(ApiError::Hotel)?;

    Ok( (StatusCode::OK, "OK"))
}