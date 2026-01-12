use config::{Config, File};
use std::env;

use super::RuntimeConfig;

pub fn build_runtime_config() -> RuntimeConfig {
    let env_name = env::var("BOOENT_ENV").unwrap_or_else(|_| "development".to_string());


    let cfg = Config::builder()
        .add_source(File::with_name(&format!("config/{}", env_name)))
        .build()
        .expect("Failed to load config");

    let database_url = cfg
        .get_string("database.url")
        .expect("database.url missing");

    let host = cfg
        .get_string("server.host")
        .unwrap_or_else(|_| "0.0.0.0".into());

    let port = cfg
        .get_int("server.port")
        .unwrap_or(3000);

    RuntimeConfig {
        database_url,
        server_addr: format!("{}:{}", host, port),
    }
}
