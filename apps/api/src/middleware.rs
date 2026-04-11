use axum::http::HeaderMap;
use crate::utils::HeaderMapExt;
use common::{errors::ApiError, domain_models::common as common_domain_models, db::sessions_interface::SessionRepository};
use runtime_config::RuntimeConfig;
use sqlx::PgPool;

pub async fn middleware(
    pool: &PgPool,
    runtime_config: RuntimeConfig,
    headers: HeaderMap,
) -> Result<common_domain_models::RequestContext, ApiError> {
    let access_token = headers.extract_access_token_from_header()?;
    let request_context = auth::user_management::jwt::verify_access_token(
        access_token,
        runtime_config.jwt_secret,
    )
    .map_err(ApiError::Auth)?;

    let session = pool
        .find_session_by_id(&request_context.session_id)
        .await
        .map_err(ApiError::Auth)?;

    match session {
        Some(s) if !s.revoked && s.expires_at > time::OffsetDateTime::now_utc() => {
            Ok(request_context)
        }
        _ => Err(ApiError::Auth(
            common::errors::AuthErrorTypes::InvalidToken.into(),
        )),
    }
}