use super::GoogleConfig;
use secrecy::Secret;
use url::Url;

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub database_url: Url,
    pub server_addr: String,
    pub jwt_secret: Secret<String>,
    pub google_config: GoogleConfig,
    pub admin_api_key: Secret<String>,
}
