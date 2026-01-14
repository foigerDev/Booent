use url::Url;

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub database_url: Url,
    pub server_addr: String,
}