use axum::{
    http::{header::SET_COOKIE, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use secrecy::ExposeSecret;

pub trait PrepareResponse {
    fn prepare_response(self, cookie: &str) -> Response;
}

impl<T: serde::Serialize> PrepareResponse for T {
    fn prepare_response(self, cookie: &str) -> Response {
        let mut response = Json(self).into_response();
        response
            .headers_mut()
            .append(SET_COOKIE, cookie.parse().unwrap());
        *response.status_mut() = StatusCode::OK;
        response
    }
}
