use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use error_stack::{self, Context};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthErrorTypes {
    InvalidToken,
    InvalidGoogleToken,
    GoogleKeysFetchFailed,
    InternalServerError,
    GoogleJWKNotFound,
    GoogleEmailNotVerified,
    DataNotFound { field_name: String },
}

impl fmt::Display for AuthErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthErrorTypes::InvalidToken => write!(f, "Invalid token"),
            AuthErrorTypes::InvalidGoogleToken => {
                write!(f, "Google authentication failed. Please try again")
            }
            AuthErrorTypes::GoogleKeysFetchFailed => {
                write!(f, "Unable to verify login. Please try later!")
            }
            AuthErrorTypes::InternalServerError => write!(f, "Server error"),
            AuthErrorTypes::GoogleJWKNotFound => write!(f, "Matching Google JWK not found"),
            AuthErrorTypes::DataNotFound { field_name } => {
                write!(f, "Data not found: {}", field_name)
            }
            AuthErrorTypes::GoogleEmailNotVerified => write!(f, "Email not verified"),
        }
    }
}

impl Context for AuthErrorTypes {}

pub enum ApiError {
    Auth(error_stack::Report<AuthErrorTypes>),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}

pub fn log_error_pretty<T>(error: &error_stack::Report<T>)
where
    T: std::fmt::Debug + Send + Sync + 'static,
{
    let debug = format!("{:#?}", error);

    let cleaned_bytes = strip_ansi_escapes::strip(debug);
    let cleaned = String::from_utf8(cleaned_bytes)
        .ok()
        .unwrap_or_else(|| "Failed to format error".to_string());

    tracing::error!("{}", cleaned);
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Auth(error) => {
                log_error_pretty(&error);
                let err = error.current_context();

                let status = match err {
                    AuthErrorTypes::InvalidGoogleToken => StatusCode::UNAUTHORIZED,
                    AuthErrorTypes::GoogleJWKNotFound => StatusCode::BAD_GATEWAY,
                    AuthErrorTypes::GoogleKeysFetchFailed => StatusCode::SERVICE_UNAVAILABLE,
                    AuthErrorTypes::InvalidToken => StatusCode::UNAUTHORIZED,
                    AuthErrorTypes::DataNotFound { .. } => StatusCode::NOT_FOUND,
                    AuthErrorTypes::GoogleEmailNotVerified => StatusCode::EXPECTATION_FAILED,
                    AuthErrorTypes::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let body = ErrorResponse {
                    error: ErrorBody {
                        code: format!("{:?}", err),
                        message: err.to_string(),
                    },
                };

                (status, Json(body)).into_response()
            }
        }
    }
}
