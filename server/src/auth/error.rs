use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("jwt error")]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("unauthorized")]
    Unauthorized,
}
