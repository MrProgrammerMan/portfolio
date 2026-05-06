use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
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
    jar: CookieJar,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token_str = match jar.get("jwt") {
        Some(t) => t.value(),
        None => return Ok(Redirect::to("/login").into_response()),
    };

    let token_result = decode::<Claims>(token_str, &state.jwt_decode, &Validation::default());

    let token = match token_result {
        Ok(token) => token,
        Err(e) => match e.kind() {
            jwtErrorKind::ExpiredSignature => return Ok(Redirect::to("/auth/refresh").into_response()),
            _ => return Err(AppError::AuthError(AuthError::JWTError(e))),
        },
    };

    match token.claims.role {
        Role::Superuser => Ok(next.run(req).await),
        #[allow(unreachable_patterns)]
        _ => Err(AppError::AuthError(AuthError::Unauthorized)),
    }
}
