use crate::{domain_models, errors};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct SessionRow {
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

impl SessionRow {
    pub fn into_domain_model(
        &self,
    ) -> Result<domain_models::auth::SessionData, error_stack::Report<errors::AuthErrorTypes>> {
        Ok(domain_models::auth::SessionData {
            id: self.id.clone(),
            user_id: self.user_id.clone(),
            refresh_token_hash: self.refresh_token_hash.clone(),
            user_agent: self.user_agent.clone(),
            ip_address: self.ip_address.clone(),
            expires_at: self.expires_at,
            revoked: self.revoked,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
    pub fn from_domain_model(session_data: &domain_models::auth::SessionData) -> Self {
        Self {
            id: session_data.id.clone(),
            user_id: session_data.user_id.clone(),
            refresh_token_hash: session_data.refresh_token_hash.clone(),
            user_agent: session_data.user_agent.clone(),
            ip_address: session_data.ip_address.clone(),
            expires_at: session_data.expires_at,
            revoked: session_data.revoked,
            created_at: session_data.created_at,
            updated_at: session_data.updated_at,
        }
    }
}
