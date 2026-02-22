use crate::routes::{health, auth};
use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};

pub fn build_app(state: Arc<crate::app_state::AppState>) -> Router {
    // let user_auth_api = Router::new().route("/identity", post(register_user)).route(
    //     "/session",
    //     post(create_session)
    //         .get(get_session)
    //         .put(renew_session)
    //         .delete(delete_session),
    // );

    Router::new()
        .route("/health", get(health::health_check))
        .route("/login",post(auth::login))
        // .nest("/auth", user_auth_api)
        .with_state(state)
}