use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use openidconnect::{
    AccessTokenHash, AuthorizationCode, Nonce, OAuth2TokenResponse, PkceCodeVerifier, TokenResponse,
};
use serde::Deserialize;
use tower_sessions::Session;

use app::state::AppState;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn auth_callback_handler(
    Query(params): Query<CallbackParams>,
    State(app_state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let stored_csrf: String = match session.get("csrf_token").await {
        Ok(Some(v)) => v,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Missing CSRF token in session".to_string(),
            ));
        }
    };

    if stored_csrf != params.state {
        return Err((StatusCode::BAD_REQUEST, "CSRF token mismatch".to_string()));
    }

    let pkce_verifier = match session.get("pkce_verifier").await {
        Ok(Some(v)) => PkceCodeVerifier::from(v),
        _ => return Err((StatusCode::BAD_REQUEST, "Missing PKCE verifier".to_string())),
    };
    let nonce = match session.get("nonce").await {
        Ok(Some(v)) => Nonce::from(v),
        _ => return Err((StatusCode::BAD_REQUEST, "Missing nonce".to_string())),
    };

    let token_response = match app_state
        .oauth_client
        .exchange_code(AuthorizationCode::new(params.code))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&app_state.http_client)
        .await
    {
        Ok(t) => t,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let id_token = token_response.id_token().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server did not return an ID token".to_string(),
        )
    })?;
    let id_token_verifier = app_state.oauth_client.id_token_verifier();
    let claims = id_token
        .claims(&id_token_verifier, &nonce)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            id_token
                .signing_alg()
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
            id_token
                .signing_key(&id_token_verifier)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        if actual_access_token_hash != *expected_access_token_hash {
            return Err((StatusCode::BAD_REQUEST, "Invalid access token".to_string()));
        }
    }

    println!(
        "User {} with e-mail address {} has authenticated successfully",
        claims.subject().as_str(),
        claims
            .email()
            .map(|email| email.as_str())
            .unwrap_or("<not provided>"),
    );

    Ok("")
}
