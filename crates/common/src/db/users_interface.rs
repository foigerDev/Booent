use crate::{domain_models, errors};
use async_trait::async_trait;
use error_stack::ResultExt;
use secrecy::Secret;

#[async_trait]
pub trait UserRepository {
    async fn find_user_by_provider_identity(
        &self,
        provider: &str,
        provider_user_id: &str,
        encryption_key: &Secret<String>,
    ) -> Result<Option<domain_models::auth::UserData>, error_stack::Report<errors::AuthErrorTypes>>;

    async fn find_user_by_user_id(
        &self,
       user_id: &str,
       encryption_key: &Secret<String>,
    ) -> Result<Option<domain_models::auth::UserData>, error_stack::Report<errors::AuthErrorTypes>>;

    async fn create_user(
        &self,
        user_data: domain_models::auth::UserData,
        encryption_key: &Secret<String>,
    ) -> Result<domain_models::auth::UserData, error_stack::Report<errors::AuthErrorTypes>>;
}

#[async_trait]
impl UserRepository for sqlx::PgPool {
    async fn find_user_by_provider_identity(
        &self,
        provider: &str,
        provider_user_id: &str,
        encryption_key: &Secret<String>,
    ) -> Result<Option<domain_models::auth::UserData>, error_stack::Report<errors::AuthErrorTypes>>
    {
        let user = sqlx::query_file_as!(
            super::models::users::UserRow,
            "src/db/queries/find_user_by_provider_identity.sql",
            provider,
            provider_user_id
        )
        .fetch_optional(self)
        .await
        .attach_printable("Database error while fetching user by provider identity")
        .change_context(errors::AuthErrorTypes::InternalServerError)?;

        let user_data = user
            .map(|user_row| user_row.into_domain_model(encryption_key))
            .transpose()?;

        Ok(user_data)
    }
    async fn find_user_by_user_id(
        
        &self,
       user_id: &str,
       encryption_key: &Secret<String>,
    ) -> Result<Option<domain_models::auth::UserData>, error_stack::Report<errors::AuthErrorTypes>> {
         let user = sqlx::query_file_as!(
            super::models::users::UserRow,
            "src/db/queries/find_user_by_user_id.sql",
            user_id
        )
        .fetch_optional(self)
        .await
        .attach_printable("Database error while fetching user by user id")
        .change_context(errors::AuthErrorTypes::InternalServerError)?;

        let user_data = user
            .map(|user_row| user_row.into_domain_model(encryption_key))
            .transpose()?;

        Ok(user_data)
    }

    async fn create_user(
        &self,
        user_data: domain_models::auth::UserData,
        encryption_key: &Secret<String>,
    ) -> Result<domain_models::auth::UserData, error_stack::Report<errors::AuthErrorTypes>> {
        let user_row =
            super::models::users::UserRow::from_domain_model(&user_data, encryption_key)?;

        let created_user = sqlx::query_file_as!(
            super::models::users::UserRow,
            "src/db/queries/add_user.sql",
            user_row.id,
            user_row.auth_provider,
            user_row.auth_provider_user_id,
            user_row.name,
            user_row.email,
            user_row.is_email_verified,
            user_row.phone,
            user_row.is_phone_verified,
            user_row.picture_url,
            user_row.status,
        )
        .fetch_one(self)
        .await
        .attach_printable("Database error while creating user")
        .change_context(errors::AuthErrorTypes::InternalServerError)?;

        Ok(created_user.into_domain_model(encryption_key)?)
    }
}
