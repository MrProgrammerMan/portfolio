use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tower_sessions::session;

use crate::auth::error::AuthError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("session storage failed: {0}")]
    SessionError(#[from] session::Error),
    #[error("authorization error")]
    AuthError(#[from] AuthError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            Self::SessionError(_) => "Session storage failed",
            Self::AuthError(_) => "Authorization failed",
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
