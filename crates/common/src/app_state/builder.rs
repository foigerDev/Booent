use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use std::time::Duration;

use crate::runtime_config::build_runtime_config;
use crate::app_state::AppState;

pub async fn build_app_state() -> AppState {
    let config = build_runtime_config();

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url.to_string())
        .await
        .expect("Failed to connect to Postgres");

    AppState {
        db: pool,
        config: Arc::new(config),
    }
}
