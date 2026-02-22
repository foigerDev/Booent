use axum::{extract::State, http::StatusCode, response::IntoResponse};
use crate::app_state::AppState;
use std::sync::Arc;

pub async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db_check = sqlx::query("SELECT 1").execute(&state.db).await;

    match db_check {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "DB DOWN"),
    }
}
