use axum::{
    body::Body,
    extract::{Query, State},
    http::{Response, StatusCode},
    response::{IntoResponse, Redirect},
};
use openidconnect::{
    AccessTokenHash, AuthorizationCode, CsrfToken, Nonce, OAuth2TokenResponse, PkceCodeChallenge,
    Scope, TokenResponse, core::CoreAuthenticationFlow,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    auth::{
        error::AuthError,
        jwt::{self, Role},
    },
    error::{AppError, RequestError},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn auth_login_handler(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token, nonce) = state
        .oauth_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    session.insert("csrf_token", csrf_token.secret()).await?;
    session.insert("nonce", nonce.secret()).await?;
    session
        .insert("pkce_verifier", pkce_verifier.secret())
        .await?;

    Ok(Redirect::to(auth_url.as_str()))
}

pub async fn auth_callback_handler(
    Query(params): Query<CallbackParams>,
    State(app_state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let stored_csrf: String = match session.get("csrf_token").await {
        Ok(Some(v)) => v,
        _ => return Err(AppError::BadRequest(RequestError::MissingSession("CSRF"))),
    };

    if stored_csrf != params.state {
        return Err(AppError::AuthError(AuthError::ValidationError(
            "CSRF token mismatch",
        )));
    }

    let pkce_verifier = match session.get("pkce_verifier").await {
        Ok(Some(v)) => v,
        _ => return Err(AppError::BadRequest(RequestError::MissingSession("PKCE"))),
    };
    let nonce = match session.get::<Nonce>("nonce").await {
        Ok(Some(v)) => v,
        _ => return Err(AppError::BadRequest(RequestError::MissingSession("Nonce"))),
    };

    let token_response = match app_state
        .oauth_client
        .exchange_code(AuthorizationCode::new(params.code))
        .map_err(|_| AuthError::ValidationError("Failed to configure OIDC exchange"))?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&app_state.http_client)
        .await
    {
        Ok(t) => t,
        Err(_) => {
            return Err(AppError::AuthError(AuthError::ServiceError(
                "HTTP error on code exchange",
            )));
        }
    };

    let id_token =
        token_response
            .id_token()
            .ok_or(AppError::AuthError(AuthError::ServiceError(
                "Server did not return an ID token",
            )))?;

    let id_token_verifier = app_state.oauth_client.id_token_verifier();

    let claims = id_token.claims(&id_token_verifier, &nonce).map_err(|_| {
        AppError::AuthError(AuthError::ValidationError(
            "Failed to verify ID token claims",
        ))
    })?;

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let signing_alg = id_token.signing_alg().map_err(|_| {
            AppError::AuthError(AuthError::ValidationError(
                "Failed to get signing algorithm",
            ))
        })?;

        let signing_key = id_token.signing_key(&id_token_verifier).map_err(|_| {
            AppError::AuthError(AuthError::ValidationError("Failed to get signing key"))
        })?;

        let actual_access_token_hash =
            AccessTokenHash::from_token(token_response.access_token(), signing_alg, signing_key)
                .map_err(|_| {
                    AppError::AuthError(AuthError::ValidationError("Failed to hash access token"))
                })?;

        if actual_access_token_hash != *expected_access_token_hash {
            return Err(AppError::AuthError(AuthError::Unauthorized));
        }
    }

    if let Some(email) = claims.email().map(|email| email.as_str())
        && email == "jonas.baugerud@gmail.com"
    {
        let token = jwt::generate(&app_state.jwt_encode, Role::Superuser, email.to_string())
            .map_err(|e| AppError::AuthError(AuthError::JWTError(e)))?;

        let html = format!(
            r#"<!DOCTYPE html>
            <html>
            <head>
                <meta http-equiv="refresh" content="0;url=/admin">
                <script>window.location.href = "/admin";</script>
            </head>
            <body>Redirecting...</body>
            </html>"#
        );

        let res = Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(
                "Set-Cookie",
                format!("token={}; HttpOnly; Secure; SameSite=Strict; Path=/", token),
            )
            .header("Content-Type", "text/html")
            .body(Body::from(html))
            .map_err(|_| AppError::AuthError(AuthError::Internal("Failed to build response")))?;

        return Ok(res);
    }

    Err(AppError::AuthError(AuthError::Unauthorized))
}
