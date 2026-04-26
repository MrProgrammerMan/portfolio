use thiserror::Error;
use tower_sessions::session;

#[derive(Error, Debug)]
enum AppError {
    #[error("session storage failed: {0}")]
    SessionError(#[from] session::Error),
}