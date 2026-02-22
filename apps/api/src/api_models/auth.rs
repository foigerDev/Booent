use serde::{Deserialize, Serialize};
use common::domain_models::auth::GoogleLoginRequest;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub id_token: String,
}

impl From<LoginRequest> for GoogleLoginRequest {
    fn from(req: LoginRequest) -> Self {
        Self { id_token: req.id_token }
    }
}