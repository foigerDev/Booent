use crate::{domain_models, encryption, errors};
use error_stack::ResultExt;
use secrecy::Secret;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: String,
    pub auth_provider: String,
    pub auth_provider_user_id: String,
    pub name: String,
    pub email: Option<String>,
    pub is_email_verified: bool,
    pub phone: Option<String>,
    pub is_phone_verified: bool,
    pub picture_url: Option<String>,
    pub status: String,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

impl UserRow {
    pub fn into_domain_model(
        &self,
        encryption_key: &Secret<String>,
    ) -> Result<domain_models::auth::UserData, error_stack::Report<errors::AuthErrorTypes>> {
        let name = encryption::decrypt_string(&self.name, encryption_key)
            .change_context(errors::AuthErrorTypes::InternalServerError)?;
        let email = self
            .email
            .as_ref()
            .map(|email| encryption::decrypt_string(email, encryption_key))
            .transpose()
            .change_context(errors::AuthErrorTypes::InternalServerError)?;
        let phone = self
            .phone
            .as_ref()
            .map(|phone| encryption::decrypt_string(phone, encryption_key))
            .transpose()
            .change_context(errors::AuthErrorTypes::InternalServerError)?;

        let picture_url = self
            .picture_url
            .as_ref()
            .map(|picture_url| encryption::decrypt_string(picture_url, encryption_key))
            .transpose()
            .change_context(errors::AuthErrorTypes::InternalServerError)?;

        Ok(domain_models::auth::UserData {
            id: self.id.clone(),
            auth_provider: self
                .auth_provider
                .parse()
                .map_err(|_| errors::AuthErrorTypes::InternalServerError)?,
            auth_provider_user_id: self.auth_provider_user_id.clone(),
            name,
            email,
            is_email_verified: self.is_email_verified,
            phone,
            is_phone_verified: self.is_phone_verified,
            picture_url,
            status: self
                .status
                .parse()
                .map_err(|_| errors::AuthErrorTypes::InternalServerError)?,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
