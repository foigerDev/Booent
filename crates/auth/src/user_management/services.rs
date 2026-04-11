use crate::user_management::{jwt, token};
use common::db::{sessions_interface::SessionRepository, users_interface::UserRepository};
use common::{consts, domain_models, errors};
use common::common_enums::Role;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

pub async fn user_auth_core(
    app_state: &PgPool,
    app_configs: &runtime_config::RuntimeConfig,
    session_id: String,
    user_id: String,
    role: Role,
) -> Result<domain_models::auth::TokenPair, error_stack::Report<errors::AuthErrorTypes>> {
    let access_token = jwt::generate_access_tokens(
        user_id.clone(),
        session_id,
        role,
        app_configs.jwt_secret.clone(),
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
    let tokens = user_auth_core(app_state, &app_configs, session_id, user.id, Role::Admin).await?;
    Ok(tokens)
}

pub async fn user_sign_up(
    app_state: &PgPool,
    app_configs: runtime_config::RuntimeConfig,
    claims: domain_models::auth::GoogleUserClaims,
) -> Result<domain_models::auth::TokenPair, error_stack::Report<errors::AuthErrorTypes>> {
    let user_data = domain_models::auth::UserData::from(claims.clone());
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
    let user = create_user(app_state, &claims, &app_configs).await?;
    let session_id = uuid::Uuid::new_v4().to_string();
    let tokens = user_auth_core(app_state, &app_configs, session_id, user.id, Role::Admin).await?;
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

pub async fn create_user(
    pool: &PgPool,
    claims: &domain_models::auth::GoogleUserClaims,
    app_configs: &runtime_config::RuntimeConfig,
) -> Result<domain_models::auth::UserData, error_stack::Report<errors::AuthErrorTypes>> {
    let user_id = uuid::Uuid::new_v4().to_string();
    let now = time::OffsetDateTime::now_utc();

    let user_data = domain_models::auth::UserData {
        id: user_id,
        auth_provider: common::common_enums::AuthProvider::Google,
        auth_provider_user_id: claims.google_user_id.clone(),
        name: claims.name.clone(),
        email: claims.email.clone(),
        is_email_verified: claims.email_verified.unwrap_or(false),
        phone: None,
        is_phone_verified: false,
        picture_url: claims.picture.clone(),
        status: common::common_enums::UserAccountStatus::Active,
        created_at: now,
        updated_at: now,
    };

    let user_output = pool
        .create_user(user_data, &app_configs.admin_api_key)
        .await?;

    Ok(user_output)
}

pub async fn refresh_tokens(
    pool: &PgPool,
    refresh_token: &str,
    jwt_secret: Secret<String>,
) -> Result<domain_models::auth::TokenPair, error_stack::Report<errors::AuthErrorTypes>> {
    let token_hash = token::hash_refresh_token(refresh_token);
    let new_refresh_token = token::generate_refresh_token();
    let new_token_hash = token::hash_refresh_token(new_refresh_token.expose_secret());

    let new_expiry = time::OffsetDateTime::now_utc()
        + time::Duration::minutes(consts::MAX_REFRESH_TOKEN_VALIDITY_IN_MINUTES);

    let session = pool
        .update_session_by_hash(&token_hash, &new_token_hash, new_expiry)
        .await?;

    let access_token = jwt::generate_access_tokens(
        session.user_id.clone(),
        session.id.clone(),
        Role::Admin,
        jwt_secret,
    )?;

    Ok(domain_models::auth::TokenPair {
        access_token,
        refresh_token: new_refresh_token,
    })
}

