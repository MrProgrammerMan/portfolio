#[cfg(feature = "ssr")]
use crate::state::AppState;
use leptos::{prelude::*, server, task::spawn_local};
use leptos_router::{NavigateOptions, hooks::use_navigate};
#[cfg(feature = "ssr")]
use openidconnect::{CsrfToken, Nonce, PkceCodeChallenge, Scope, core::CoreAuthenticationFlow};
#[cfg(feature = "ssr")]
use tower_sessions::Session;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <h1>"The login page"</h1>
        <button on:click=move |_| {
            let navigate = use_navigate();
            spawn_local(async move {
                if let Ok(url) = login().await {
                    navigate(&url, NavigateOptions::default());
                }
            });
        }>"Log in with Google"</button>
    }
}

#[server]
pub async fn login() -> Result<String, ServerFnError> {
    let state = use_context::<AppState>().unwrap();

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

    let session: Session = leptos_axum::extract().await?;

    session.insert("csrf_token", csrf_token.secret()).await?;
    session.insert("nonce", nonce.secret()).await?;
    session
        .insert("pkce_verifier", pkce_verifier.secret())
        .await?;

    Ok(auth_url.to_string())
}
