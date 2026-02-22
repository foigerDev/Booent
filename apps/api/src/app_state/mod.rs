pub mod builder;

use runtime_config::RuntimeConfig;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: RuntimeConfig,
}
