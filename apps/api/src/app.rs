use crate::routes::{admin, amenities, auth, health, hotels};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn build_app(state: Arc<crate::app_state::AppState>) -> Router {
    let user_routes = Router::new()
        .route("/login", post(auth::login))
        .route("/signup", post(auth::signup))
        .route("/refresh", post(auth::refresh))
        .with_state(state.clone());

    let hotel_routes = Router::new()
        .route("/", post(hotels::hotel_create))
        .route("/:hotel_id", put(hotels::hotel_update))
        .route("/:hotel_id/branding", put(hotels::hotel_branding_update))
        .route("/:hotel_id/room_types", post(hotels::create_room_type))
        .route(
            "/:hotel_id/room_types/:room_type_id/amenities",
            put(hotels::update_room_type_amenities),
        )
        .route(
            "/:hotel_id/room_types/:room_type_id/images",
            post(hotels::add_room_type_image),
        )
        .route(
            "/:hotel_id/room_types/:room_type_id/images",
            get(hotels::get_room_type_images),
        )
        .route(
            "/:hotel_id/room_types/:room_type_id/images/:image_id",
            delete(hotels::delete_room_type_image),
        )
        .route("/amenities", get(amenities::get_hotel_amenities))
        .route("/room_amenities", get(amenities::get_room_amenities))
        // .route("/", get(hotels::hotel_list))
        // .route("/:hotel_id", get(hotels::hotel_retrieve))
        .with_state(state.clone());

    let admin_routes = Router::new()
        .route("/hotel/status", post(admin::update_status))
        .with_state(state.clone());

    Router::new()
        .route("/health", get(health::health_check))
        .nest("/user", user_routes)
        .nest("/hotel", hotel_routes)
        .nest("/admin", admin_routes)
        .with_state(state)
}
