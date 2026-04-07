use crate::{api_models::admin as api_models_admin, app_state::AppState, utils};
use admin::services::update_hotel_status;
use axum::{
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
    Json,
};
use common::errors::ApiError;
use secrecy::ExposeSecret;
use std::sync::Arc;


pub async fn update_status(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<api_models_admin::UpdateHotelStatusRequest>,
) -> Result<impl IntoResponse, ApiError> {
    utils::validate_api_key(&headers, state.config.admin_api_key.expose_secret())?;

    let hotel = update_hotel_status(&state.db, payload.hotel_id, payload.status)
        .await
        .map_err(ApiError::Hotel)?;

    let response = api_models_admin::UpdateHotelStatusResponse {
        status: hotel.status,
    };

    Ok(Json(response))
}
