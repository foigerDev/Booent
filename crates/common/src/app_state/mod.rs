pub mod builder;

use sqlx::PgPool;
use std::sync::Arc;
use crate::runtime_config::RuntimeConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<RuntimeConfig>,
}
