use crate::{api_models::amenities as api_models_amenities, app_state::AppState, utils};
use axum::{
    extract::State,
    http::HeaderMap,
    Json,
};
use common::errors::ApiError;
use std::sync::Arc;
use secrecy::ExposeSecret;


pub async fn get_hotel_amenities(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<api_models_amenities::HotelAmenitiesResponse>, ApiError> {
    utils::validate_api_key(&headers, state.config.admin_api_key.expose_secret())?;
    let amenities = hotels::amenities::get_hotel_amenities(&state.db)
        .await
        .map_err(ApiError::Hotel)?;
    
    let response = api_models_amenities::HotelAmenitiesResponse {
        amenities: amenities.into_iter().map(api_models_amenities::AmenityResponse::from).collect(),
    };

    Ok(Json(response))
}
