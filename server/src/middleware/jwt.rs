use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Validation, decode};

use crate::{
    auth::jwt::{Claims, Role},
    error::AppError,
    state::AppState,
};

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
        #[allow(unreachable_patterns)]
        _ => Err(AppError::AuthError),
    }
}
