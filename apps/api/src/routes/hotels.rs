use crate::{api_models::hotels as api_models_hotels, app_state::AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::{errors::{ self, ApiError}, db::users_interface::UserRepository, db::hotels_interface::HotelRepository};
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

pub async fn hotel_update(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(hotel_id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<api_models_hotels::HotelUpdateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let request_context = middleware::middleware(&state.db, state.config.clone(), headers).await?;
    let _user = state.db.find_user_by_user_id(&request_context.user_id, &state.config.admin_api_key).await.change_context
    (errors::AuthErrorTypes::UserNotFound).map_err(ApiError::Auth)?;
    let hotel_response = hotels::services::update_hotel(&state.db, hotel_id, payload.into()).await.map_err(ApiError::Hotel)?;
    let response_body = api_models_hotels::HotelUpdateResponse::from(hotel_response);

    Ok(Json(response_body))
}

pub async fn hotel_branding_update(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(hotel_id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<api_models_hotels::HotelBrandingUpdateRequest>,
) -> Result<Json<api_models_hotels::HotelBrandingUpdateResponse>, ApiError> {
    let request_context = middleware::middleware(&state.db, state.config.clone(), headers).await?;
    let _user = state.db.find_user_by_user_id(&request_context.user_id, &state.config.admin_api_key).await.change_context
    (errors::AuthErrorTypes::UserNotFound).map_err(ApiError::Auth)?;
    
    let user_owns_hotel = state.db.check_user_owns_hotel(&request_context.user_id, hotel_id).await
        .map_err(ApiError::Hotel)?;
    if !user_owns_hotel {
        return Err(ApiError::Hotel(
            error_stack::Report::new(errors::HotelErrorTypes::UnauthorizedHotelAccess)
        ));
    }
    
    let branding_data = hotels::services::update_hotel_branding(&state.db, hotel_id, payload.into()).await.map_err(ApiError::Hotel)?;
    let response_body = api_models_hotels::HotelBrandingUpdateResponse::from(branding_data);
    
    Ok(Json(response_body))
}

pub async fn create_room_type(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(hotel_id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<api_models_hotels::RoomTypeCreateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let request_context = middleware::middleware(&state.db, state.config.clone(), headers).await?;
    let _user = state.db.find_user_by_user_id(&request_context.user_id, &state.config.admin_api_key).await.change_context
    (errors::AuthErrorTypes::UserNotFound).map_err(ApiError::Auth)?;
    
    let user_owns_hotel = state.db.check_user_owns_hotel(&request_context.user_id, hotel_id).await
        .map_err(ApiError::Hotel)?;
    if !user_owns_hotel {
        return Err(ApiError::Hotel(
            error_stack::Report::new(errors::HotelErrorTypes::UnauthorizedHotelAccess)
        ));
    }
    
    let room_type = hotels::services::create_room_type(&state.db, hotel_id, payload.into()).await.map_err(ApiError::Hotel)?;
    let response_body = api_models_hotels::RoomTypeResponse::from(room_type);

    let mut response = Json(response_body).into_response();
    *response.status_mut() = StatusCode::CREATED;

    Ok(response)
}