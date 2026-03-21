use crate::user_management::{jwt, token};
use common::db::{sessions_interface::SessionRepository, users_interface::UserRepository};
use common::{domain_models, errors};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

pub async fn user_auth_core(
    app_state: &PgPool,
    app_configs: runtime_config::RuntimeConfig,
    session_id: String,
    user_id: String,
) -> Result<domain_models::auth::TokenPair, error_stack::Report<errors::AuthErrorTypes>> {
    let access_token = jwt::generate_access_tokens(
        user_id.clone(),
        session_id,
        app_configs.admin_api_key.clone(),
    )?;
    let refresh_token = token::generate_refresh_token();
    create_session(app_state, user_id, refresh_token.clone()).await?;

    Ok(domain_models::auth::TokenPair {
        access_token,
        refresh_token,
    })
}

pub async fn user_login(
    app_state: &PgPool,
    app_configs: runtime_config::RuntimeConfig,
    claims: domain_models::auth::GoogleUserClaims,
) -> Result<domain_models::auth::TokenPair, error_stack::Report<errors::AuthErrorTypes>> {
    let user_data = common::domain_models::auth::UserData::from(claims);
    let user = app_state
        .find_user_by_provider_identity(
            &user_data.auth_provider.to_string(),
            user_data.auth_provider_user_id.as_str(),
            &app_configs.admin_api_key,
        )
        .await?;
    let user = user.ok_or(errors::AuthErrorTypes::UserNotFound)?;
    let session_id = uuid::Uuid::new_v4().to_string();
    let tokens = user_auth_core(app_state, app_configs, session_id, user.id).await?;
    Ok(tokens)
}

pub async fn user_sign_up(
    app_state: &PgPool,
    app_configs: runtime_config::RuntimeConfig,
    claims: domain_models::auth::GoogleUserClaims,
) -> Result<domain_models::auth::TokenPair, error_stack::Report<errors::AuthErrorTypes>> {
    let user_data = domain_models::auth::UserData::from(claims);
    let user = app_state
        .find_user_by_provider_identity(
            &user_data.auth_provider.to_string(),
            user_data.auth_provider_user_id.as_str(),
            &app_configs.admin_api_key,
        )
        .await?;
    if user.is_some() {
        return Err(errors::AuthErrorTypes::UserAlreadyRegistered.into());
    }
    let user_id = uuid::Uuid::new_v4().to_string();
    let session_id = uuid::Uuid::new_v4().to_string();
    let tokens = user_auth_core(app_state, app_configs, session_id, user_id).await?;

    Ok(tokens)
}

pub async fn create_session(
    pool: &PgPool,
    user_id: String,
    refresh_token: Secret<String>,
) -> Result<domain_models::auth::SessionData, error_stack::Report<errors::AuthErrorTypes>> {
    let now = time::OffsetDateTime::now_utc();
    let expires_at =
        now + time::Duration::minutes(common::consts::MAX_REFRESH_TOKEN_VALIDITY_IN_MINUTES);

    let session = domain_models::auth::SessionData {
        id: uuid::Uuid::new_v4().to_string(),
        user_id,
        refresh_token_hash: token::hash_refresh_token(refresh_token.expose_secret()),
        expires_at,
        revoked: false,
        user_agent: None,
        created_at: now,
        updated_at: now,
        ip_address: None,
    };

    let session_output = pool.create_session(session).await?;

    Ok(session_output)
}
