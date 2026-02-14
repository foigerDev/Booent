use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub async fn health_check(State(state): State<common::app_state::AppState>) -> impl IntoResponse {
    let db_check = sqlx::query("SELECT 1").execute(&state.db).await;

    match db_check {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "DB DOWN"),
    }
}
