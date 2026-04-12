use crate::routes::{admin, auth, health, hotels};
use axum::{
    routing::{get, post, put},
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
