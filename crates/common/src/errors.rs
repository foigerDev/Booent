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
    ApiAuthorizationFailed,
    UserAlreadyRegistered,
    UserNotFound,
    RefreshTokenInvalid,
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
            AuthErrorTypes::ApiAuthorizationFailed => write!(f, "Unauthorized access"),
            AuthErrorTypes::UserAlreadyRegistered => {
                write!(f, "User already registered! Please loggin")
            }
            AuthErrorTypes::UserNotFound => write!(f, "User not found"),
            AuthErrorTypes::RefreshTokenInvalid => write!(f, "Invalid or expired refresh token"),
        }
    }
}

impl Context for AuthErrorTypes {}

#[derive(Debug, Serialize, Deserialize)]
pub enum EncryptionErrorTypes {
    EncryptionFailed,
    DecryptionFailed,
}

impl fmt::Display for EncryptionErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionErrorTypes::EncryptionFailed => write!(f, "Encryption failed"),
            EncryptionErrorTypes::DecryptionFailed => write!(f, "Decryption failed"),
        }
    }
}

impl Context for EncryptionErrorTypes {}

#[derive(Debug, Serialize, Deserialize)]
pub enum HotelErrorTypes {
    HotelCreationFailed,
    HotelAlreadyExists,
    HotelNotFound,
    UnauthorizedHotelAccess,
    InternalServerError,
}

impl fmt::Display for HotelErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HotelErrorTypes::HotelCreationFailed => write!(f, "Hotel creation failed"),
            HotelErrorTypes::HotelAlreadyExists => write!(f, "Hotel already registered"),
            HotelErrorTypes::HotelNotFound => write!(f, "Hotel not found"),
            HotelErrorTypes::UnauthorizedHotelAccess => {
                write!(f, "You don't have access to this hotel")
            }
            HotelErrorTypes::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl Context for HotelErrorTypes {}

pub enum ApiError {
    Auth(error_stack::Report<AuthErrorTypes>),
    Encryption(error_stack::Report<EncryptionErrorTypes>),
    Hotel(error_stack::Report<HotelErrorTypes>),
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
                    AuthErrorTypes::ApiAuthorizationFailed => StatusCode::UNAUTHORIZED,
                    AuthErrorTypes::UserAlreadyRegistered => StatusCode::BAD_REQUEST,
                    AuthErrorTypes::UserNotFound => StatusCode::NOT_FOUND,
                    AuthErrorTypes::RefreshTokenInvalid => StatusCode::UNAUTHORIZED,
                };

                let body = ErrorResponse {
                    error: ErrorBody {
                        code: format!("{:?}", err),
                        message: err.to_string(),
                    },
                };

                (status, Json(body)).into_response()
            }
            ApiError::Encryption(error) => {
                log_error_pretty(&error);
                let err = error.current_context();
                let status = match err {
                    EncryptionErrorTypes::EncryptionFailed
                    | EncryptionErrorTypes::DecryptionFailed => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let body = ErrorResponse {
                    error: ErrorBody {
                        code: format!("{:?}", err),
                        message: err.to_string(),
                    },
                };

                (status, Json(body)).into_response()
            }
            ApiError::Hotel(error) => {
                log_error_pretty(&error);
                let err = error.current_context();
                let status = match err {
                    HotelErrorTypes::HotelCreationFailed => StatusCode::INTERNAL_SERVER_ERROR,
                    HotelErrorTypes::HotelAlreadyExists => StatusCode::CONFLICT,
                    HotelErrorTypes::HotelNotFound => StatusCode::NOT_FOUND,
                    HotelErrorTypes::UnauthorizedHotelAccess => StatusCode::FORBIDDEN,
                    HotelErrorTypes::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
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
