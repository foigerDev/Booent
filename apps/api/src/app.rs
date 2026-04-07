use crate::routes::{auth, health, hotels};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn build_app(state: Arc<crate::app_state::AppState>) -> Router {
    let user_routes = Router::new()
        .route("/login", post(auth::login))
        .route("/signup", post(auth::signup))
        .with_state(state.clone());

   let hotel_routes = Router::new()
    .route("/", post(hotels::hotel_create).get(hotels::hotel_create))
    // .route("/:hotel_id", get(hotels::hotel_retrieve))
    .with_state(state.clone());

    Router::new()
        .route("/health", get(health::health_check))
        .nest("/user", user_routes)
        .nest("/hotel", hotel_routes)
        .with_state(state)
}
