use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GoogleConfig {
    pub client_id: String,
}
