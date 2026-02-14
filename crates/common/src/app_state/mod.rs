pub mod builder;

use crate::runtime_config::RuntimeConfig;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<RuntimeConfig>,
}
