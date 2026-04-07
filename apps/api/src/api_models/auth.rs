use common::domain_models::auth;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub id_token: String,
}

impl From<LoginRequest> for auth::GoogleLoginRequest {
    fn from(req: LoginRequest) -> Self {
        Self {
            id_token: req.id_token,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
}

impl From<auth::TokenPair> for LoginResponse {
    fn from(tokens: auth::TokenPair) -> Self {
        Self {
            access_token: tokens.access_token.expose_secret().clone(),
        }
    }
}

impl From<(secrecy::Secret<String>, secrecy::Secret<String>)> for LoginResponse {
    fn from(tokens: (secrecy::Secret<String>, secrecy::Secret<String>)) -> Self {
        Self {
            access_token: tokens.0.expose_secret().clone(),
        }
    }
}
