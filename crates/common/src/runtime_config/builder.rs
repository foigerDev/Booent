use config::{Config, File};
use std::env;

use super::{AppConfig, RuntimeConfig};

pub fn build_runtime_config() -> RuntimeConfig {
    // Select environment
    let env_name = env::var("BOOENT_ENV")
        .unwrap_or_else(|_| "development".to_string());

    // Load config file
    let cfg = Config::builder()
        .add_source(File::with_name(&format!("config/{}", env_name)))
        .build()
        .expect("❌ Failed to load config file");

    // Deserialize & validate
    let app_config: AppConfig = cfg
        .try_deserialize()
        .expect("❌ Invalid configuration format");

    let db_url = app_config.database.to_url();

    RuntimeConfig {
        database_url: db_url,
        server_addr: format!(
            "{}:{}",
            app_config.server.host,
            app_config.server.port
        ),
    }
}

