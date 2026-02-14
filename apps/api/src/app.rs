use crate::routes::health;
use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn build_app(state: common::app_state::AppState) -> Router {
    // let user_auth_api = Router::new().route("/identity", post(register_user)).route(
    //     "/session",
    //     post(create_session)
    //         .get(get_session)
    //         .put(renew_session)
    //         .delete(delete_session),
    // );

    Router::new()
        .route("/health", get(health::health_check))
        // .nest("/auth", user_auth_api)
        .with_state(state)
}