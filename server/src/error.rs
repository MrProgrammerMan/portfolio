use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tower_sessions::session;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("session storage failed: {0}")]
    SessionError(#[from] session::Error),
    #[error("authorization error")]
    AuthError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            Self::SessionError(_) => "Session storage failed",
            Self::AuthError => "Authorization failed",
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
