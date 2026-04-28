use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use jsonwebtoken::{Validation, decode, errors::ErrorKind as jwtErrorKind};

use crate::{
    auth::{
        error::AuthError,
        jwt::{Claims, Role},
    },
    error::AppError,
    state::AppState,
};

pub async fn jwt_validation(
    State(state): State<AppState>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let cookie_header = match headers.get("cookie") {
        Some(h) => h,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    let token_str = cookie_header
        .to_str()
        .map_err(|_| {
            AppError::AuthError(AuthError::ValidationError("Failed to parse cookie header"))
        })?
        .split(';')
        .find(|c| c.trim().starts_with("token="));

    let token_str = match token_str {
        Some(t) => t.trim().trim_start_matches("token="),
        None => return Ok(Redirect::to("/login").into_response()),
    };

    let token_result = decode::<Claims>(token_str, &state.jwt_decode, &Validation::default());

    let token = match token_result {
        Ok(token) => token,
        Err(e) => match e.kind() {
            jwtErrorKind::ExpiredSignature => return Ok(Redirect::to("/login").into_response()), // TODO: refresh token
            _ => return Err(AppError::AuthError(AuthError::JWTError(e))),
        },
    };

    match token.claims.role {
        Role::Superuser => Ok(next.run(req).await),
        #[allow(unreachable_patterns)]
        _ => Err(AppError::AuthError(AuthError::Unauthorized)),
    }
}
