use axum::http::HeaderMap;
use crate::utils::HeaderMapExt;
use common::{errors::ApiError, domain_models::common as common_domain_models};
use runtime_config::RuntimeConfig;


pub fn middleware(runtime_config: RuntimeConfig, headers: HeaderMap) ->  Result<common_domain_models::RequestContext, ApiError> {
    let access_token = headers.extract_access_token_from_header()?;
    print!("access token: {}", access_token);
    let request_context = auth::user_management::jwt::verify_access_token(access_token, runtime_config.jwt_secret).map_err(ApiError::Auth)?;
    // fetch session and validate session is active and not expired
    Ok(request_context)
}