use crate::{api_models::hotels as api_models_hotels, app_state::AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::{errors::{ self, ApiError}, db::users_interface::UserRepository};
use error_stack::ResultExt;
use std::sync::Arc;
use crate::middleware;

pub async fn hotel_create(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<api_models_hotels::HotelCreateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let request_context = middleware::middleware(&state.db, state.config.clone(), headers).await?;
    let _user = state.db.find_user_by_user_id(&request_context.user_id, &state.config.admin_api_key).await.change_context
    (errors::AuthErrorTypes::UserNotFound).map_err(ApiError::Auth)?;
    let hotel_response = hotels::services::create_hotel(&state.db, payload.into()).await.map_err(ApiError::Hotel)?;
    hotels::services::add_user_to_hotel(&state.db, &request_context.user_id, hotel_response.id).await.map_err(ApiError::Hotel)?;
        let response_body = api_models_hotels::HotelCreateResponse::from(hotel_response);

    let mut response = Json(response_body).into_response();
    *response.status_mut() = StatusCode::CREATED;

    Ok(response)
}