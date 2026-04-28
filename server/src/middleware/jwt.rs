use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{
    Validation, decode, errors::Error as jwtError, errors::ErrorKind as jwtErrorKind,
};

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
    let token_header = match headers.get("Authorization") {
        Some(h) => h,
        None => todo!("Missing JWT, redirect to login"),
    };

    let token_str = token_header
        .to_str()
        .map_err(|_| app_err(jwtErrorKind::InvalidToken))?
        .strip_prefix("Bearer ")
        .ok_or(app_err(jwtErrorKind::InvalidToken))?;

    let token_result = decode::<Claims>(token_str, &state.jwt_decode, &Validation::default());

    let token = match token_result {
        Ok(token) => token,
        Err(e) => match e.kind() {
            jwtErrorKind::ExpiredSignature => todo!("Refresh token"),
            _ => return Err(AppError::AuthError(AuthError::JWTError(e))),
        },
    };

    match token.claims.role {
        Role::Superuser => Ok(next.run(req).await),
        #[allow(unreachable_patterns)]
        _ => Err(AppError::AuthError(AuthError::Unauthorized)),
    }
}

fn app_err(e: jwtErrorKind) -> AppError {
    AppError::AuthError(AuthError::JWTError(jwtError::from(e)))
}
