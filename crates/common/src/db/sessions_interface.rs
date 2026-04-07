use crate::{db::models::sessions, domain_models, errors};
use async_trait::async_trait;
use error_stack::ResultExt;

#[async_trait]
pub trait SessionRepository {
    async fn create_session(
        &self,
        session_data: domain_models::auth::SessionData,
    ) -> Result<domain_models::auth::SessionData, error_stack::Report<errors::AuthErrorTypes>>;

    async fn update_session(
        &self,
        session_id: &str,
        refresh_token_hash: &str,
        expires_at: time::OffsetDateTime,
    ) -> Result<domain_models::auth::SessionData, error_stack::Report<errors::AuthErrorTypes>>;

    async fn update_session_by_hash(
        &self,
        token_hash: &str,
        new_token_hash: &str,
        new_expires_at: time::OffsetDateTime,
    ) -> Result<domain_models::auth::SessionData, error_stack::Report<errors::AuthErrorTypes>>;
}

// todo change error types to database error type

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

    async fn update_session(
        &self,
        session_id: &str,
        refresh_token_hash: &str,
        expires_at: time::OffsetDateTime,
    ) -> Result<domain_models::auth::SessionData, error_stack::Report<errors::AuthErrorTypes>> {
        let session = sqlx::query_file_as!(
            sessions::SessionRow,
            "src/db/queries/update_session.sql",
            refresh_token_hash,
            expires_at,
            session_id,
        )
        .fetch_optional(self)
        .await
        .attach_printable("Database error while updating session")
        .change_context(errors::AuthErrorTypes::InternalServerError)?;

        match session {
            Some(s) => s.into_domain_model(),
            None => Err(error_stack::Report::new(errors::AuthErrorTypes::RefreshTokenInvalid)),
        }
    }

    async fn update_session_by_hash(
        &self,
        token_hash: &str,
        new_token_hash: &str,
        new_expires_at: time::OffsetDateTime,
    ) -> Result<domain_models::auth::SessionData, error_stack::Report<errors::AuthErrorTypes>> {
        let session = sqlx::query_file_as!(
            sessions::SessionRow,
            "src/db/queries/update_session_by_hash.sql",
            new_token_hash,
            new_expires_at,
            token_hash,
        )
        .fetch_optional(self)
        .await
        .attach_printable("Database error while updating session by hash")
        .change_context(errors::AuthErrorTypes::InternalServerError)?;

        match session {
            Some(s) => s.into_domain_model(),
            None => Err(error_stack::Report::new(errors::AuthErrorTypes::RefreshTokenInvalid)),
        }
    }
}
