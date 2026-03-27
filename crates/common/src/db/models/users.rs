use crate::{domain_models, encryption, errors};
use error_stack::ResultExt;
use secrecy::{ExposeSecret, Secret};
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

    pub fn from_domain_model(
        user_data: &domain_models::auth::UserData,
        encryption_key: &Secret<String>,
    ) -> Result<Self, error_stack::Report<errors::AuthErrorTypes>> {
        let name = encryption::encrypt_string(user_data.name.expose_secret(), encryption_key)
            .change_context(errors::AuthErrorTypes::InternalServerError)?;

        let email = user_data
            .email
            .as_ref()
            .map(|email| encryption::encrypt_string(email.expose_secret(), encryption_key))
            .transpose()
            .change_context(errors::AuthErrorTypes::InternalServerError)?;

        let phone = user_data
            .phone
            .as_ref()
            .map(|phone| encryption::encrypt_string(phone.expose_secret(), encryption_key))
            .transpose()
            .change_context(errors::AuthErrorTypes::InternalServerError)?;

        let picture_url = user_data
            .picture_url
            .as_ref()
            .map(|picture_url| {
                encryption::encrypt_string(picture_url.expose_secret(), encryption_key)
            })
            .transpose()
            .change_context(errors::AuthErrorTypes::InternalServerError)?;

        Ok(Self {
            id: user_data.id.clone(),
            auth_provider: user_data.auth_provider.to_string(),
            auth_provider_user_id: user_data.auth_provider_user_id.clone(),
            name,
            email,
            is_email_verified: user_data.is_email_verified,
            phone,
            is_phone_verified: user_data.is_phone_verified,
            picture_url,
            status: user_data.status.to_string(),
            created_at: user_data.created_at,
            updated_at: user_data.updated_at,
        })
    }
}
