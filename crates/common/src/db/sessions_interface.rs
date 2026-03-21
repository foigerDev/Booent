use crate::{db::models::sessions, domain_models, errors};
use async_trait::async_trait;
use error_stack::ResultExt;

#[async_trait]
pub trait SessionRepository {
    async fn create_session(
        &self,
        session_data: domain_models::auth::SessionData,
    ) -> Result<domain_models::auth::SessionData, error_stack::Report<errors::AuthErrorTypes>>;
}

#[async_trait]
impl SessionRepository for sqlx::PgPool {
    async fn create_session(
        &self,
        session_data: domain_models::auth::SessionData,
    ) -> Result<domain_models::auth::SessionData, error_stack::Report<errors::AuthErrorTypes>> {
        let session_data = sessions::SessionRow::from_domain_model(&session_data);
        let sessions = sqlx::query_file_as!(
            sessions::SessionRow,
            "src/db/queries/create_session.sql",
            session_data.id,
            session_data.user_id,
            session_data.refresh_token_hash,
            session_data.user_agent,
            session_data.ip_address,
            session_data.expires_at,
            session_data.revoked,
        )
        .fetch_one(self)
        .await
        .attach_printable("Database error while fetching user by provider identity")
        .change_context(errors::AuthErrorTypes::InternalServerError)?;

        let sessions_output = sessions.into_domain_model()?;

        Ok(sessions_output)
    }
}
