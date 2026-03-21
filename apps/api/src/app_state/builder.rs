use crate::app_state::AppState;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use std::time::Duration;

pub async fn build_app_state() -> Arc<AppState> {
    let config = runtime_config::build_runtime_config();
    tracing::info!("Loaded Run Time Configs");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(config.database_url.as_ref())
        .await
        .expect("Failed to connect to Postgres");

    Arc::new(AppState { db: pool, config })
}
