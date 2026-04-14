use crate::{api_models::hotels as api_models_hotels, app_state::AppState, utils};
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

pub async fn update_room_type_amenities(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    axum::extract::Path((hotel_id, room_type_id)): axum::extract::Path<(uuid::Uuid, uuid::Uuid)>,
    Json(payload): Json<api_models_hotels::RoomTypeAmenitiesUpdateRequest>,
) -> Result<Json<api_models_hotels::RoomTypeAmenitiesUpdateResponse>, ApiError> {
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
    
    let amenity_ids: Vec<uuid::Uuid> = payload.amenity_ids
        .iter()
        .filter_map(|id| uuid::Uuid::parse_str(id).ok())
        .collect();
    
    hotels::services::update_room_type_amenities(&state.db, room_type_id, amenity_ids.clone()).await.map_err(ApiError::Hotel)?;
    
    Ok(Json(api_models_hotels::RoomTypeAmenitiesUpdateResponse {
        room_type_id: room_type_id.to_string(),
        amenity_ids: amenity_ids.iter().map(|id| id.to_string()).collect(),
    }))
}

pub async fn add_room_type_image(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    axum::extract::Path((hotel_id, room_type_id)): axum::extract::Path<(uuid::Uuid, uuid::Uuid)>,
    Json(payload): Json<api_models_hotels::RoomTypeImagesUpdateRequest>,
) -> Result<Json<api_models_hotels::RoomTypeImageResponse>, ApiError> {
    let request_context = middleware::middleware(&state.db, state.config.clone(), headers).await?;
    
    utils::validate_user_owns_hotel(&state.db, &state.config.admin_api_key, &request_context.user_id, hotel_id).await?;
    
    let image = hotels::services::add_room_type_image(
        &state.db, 
        room_type_id, 
        payload.image_url, 
        payload.image_type,
        payload.display_order 
    )
        .await
        .map_err(ApiError::Hotel)?;
    
    Ok(Json(api_models_hotels::RoomTypeImageResponse::from(image)))
}

pub async fn get_room_type_images(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    axum::extract::Path((_hotel_id, room_type_id)): axum::extract::Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<Json<Vec<api_models_hotels::RoomTypeImageResponse>>, ApiError> {
    let _request_context = middleware::middleware(&state.db, state.config.clone(), headers).await?;
    
    let images = hotels::services::get_room_type_images(&state.db, room_type_id)
        .await
        .map_err(ApiError::Hotel)?;
    
    Ok(Json(images.into_iter().map(api_models_hotels::RoomTypeImageResponse::from).collect()))
}

pub async fn delete_room_type_image(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    axum::extract::Path((hotel_id, _room_type_id, image_id)): axum::extract::Path<(uuid::Uuid, uuid::Uuid, uuid::Uuid)>,
) -> Result<impl IntoResponse, ApiError> {
    let request_context = middleware::middleware(&state.db, state.config.clone(), headers).await?;
    
    utils::validate_user_owns_hotel(&state.db, &state.config.admin_api_key, &request_context.user_id, hotel_id).await?;
    
    hotels::services::delete_room_type_image(&state.db, image_id)
        .await
        .map_err(ApiError::Hotel)?;
    
    let mut response = StatusCode::NO_CONTENT.into_response();
    *response.status_mut() = StatusCode::NO_CONTENT;
    
    Ok(response)
}