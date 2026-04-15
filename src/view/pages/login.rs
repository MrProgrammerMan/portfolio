use leptos::{prelude::*, server, task::spawn_local};
use leptos_router::{NavigateOptions, hooks::use_navigate};
#[cfg(feature = "ssr")]
use openidconnect::{
    ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, PkceCodeChallenge, RedirectUrl, Scope,
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest,
};
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
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID should be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET should be set");

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Reqwest client should build");

    let issuer = IssuerUrl::new("https://accounts.google.com".to_string())
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let provider_metadata = CoreProviderMetadata::discover_async(issuer, &http_client)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:3000/auth/callback".to_string())
            .map_err(|e| ServerFnError::new(e.to_string()))?,
    );

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token, nonce) = client
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
