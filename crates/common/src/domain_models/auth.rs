use secrecy::Secret;
use serde::{Deserialize, Serialize};

use crate::common_enums;

#[derive(Debug, Deserialize, Clone)]
pub struct GoogleUserClaims {
    #[serde(rename = "sub")]
    pub google_user_id: String,
    pub email: Option<Secret<String>>,
    pub email_verified: Option<bool>,
    pub name: Secret<String>,
    pub picture: Option<Secret<String>>,
}

pub struct GoogleLoginRequest {
    pub id_token: String,
}

pub struct LoginData {
    pub access_token: Secret<String>,
    pub refresh_token: Secret<String>,
}

pub struct UserData {
    pub id: String,
    pub auth_provider: common_enums::AuthProvider,
    pub auth_provider_user_id: String,
    pub name: Secret<String>,
    pub email: Option<Secret<String>>,
    pub is_email_verified: bool,
    pub phone: Option<Secret<String>>,
    pub is_phone_verified: bool,
    pub picture_url: Option<Secret<String>>,
    pub status: common_enums::UserAccountStatus,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

impl From<GoogleUserClaims> for UserData {
    fn from(claims: GoogleUserClaims) -> Self {
        let now = time::OffsetDateTime::now_utc();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            auth_provider: common_enums::AuthProvider::Google,
            auth_provider_user_id: claims.google_user_id,
            name: claims.name,
            email: claims.email,
            is_email_verified: claims.email_verified.unwrap_or(false),
            phone: None,
            is_phone_verified: false,
            picture_url: claims.picture,
            status: common_enums::UserAccountStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub sub: String, // user_id
    pub sid: String, // session_id
    pub iss: String,
    pub aud: String,
    pub role: common_enums::Role,
    #[serde(with = "time::serde::timestamp")]
    pub exp: time::OffsetDateTime,
    #[serde(with = "time::serde::timestamp")]
    pub iat: time::OffsetDateTime,
    #[serde(with = "time::serde::timestamp")]
    pub nbf: time::OffsetDateTime,
}

pub struct TokenPair {
    pub access_token: Secret<String>,
    pub refresh_token: Secret<String>,
}

#[derive(Debug)]
pub struct SessionData {
    pub id: String,
    pub user_id: String,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub expires_at: time::OffsetDateTime,
    pub revoked: bool,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}
