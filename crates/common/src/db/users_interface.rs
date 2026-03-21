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
            "src/db/queries/find_user_by_identity.sql",
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
}
