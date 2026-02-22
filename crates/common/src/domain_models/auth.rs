use serde::Deserialize;
use secrecy::Secret;

#[derive(Debug, Deserialize)]
pub struct GoogleUserClaims {

    #[serde(rename = "sub")]
    pub google_user_id: String,

    pub email: Option<Secret<String>>,

    pub email_verified: Option<bool>,

    pub name: Option<Secret<String>>,

    pub picture: Option<String>,
}

pub struct GoogleLoginRequest {
    pub id_token: String,
}

