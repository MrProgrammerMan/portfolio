use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Validation, decode};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, state::AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: Role,
    iss: usize,
    exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Role {
    Superuser,
}

pub async fn jwt_validation(
    State(state): State<AppState>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token_header = headers.get("Authorization").ok_or(AppError::AuthError)?;

    let token_str = token_header
        .to_str()
        .map_err(|_| AppError::AuthError)?
        .strip_prefix("Bearer ")
        .ok_or(AppError::AuthError)?;

    let token = decode::<Claims>(token_str, &state.jwt_decode, &Validation::default())
        .map_err(|_| AppError::AuthError)?;

    match token.claims.role {
        Role::Superuser => Ok(next.run(req).await),
        _ => Err(AppError::AuthError),
    }
}
