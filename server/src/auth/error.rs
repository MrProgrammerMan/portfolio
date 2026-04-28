use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("jwt error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("unauthorized")]
    Unauthorized,
    #[error("Validation error: {0}")]
    ValidationError(&'static str),
    #[error("external service: {0}")]
    ServiceError(&'static str),
}
