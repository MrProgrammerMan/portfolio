use axum::extract::FromRef;
use jsonwebtoken::{DecodingKey, EncodingKey};
use leptos::config::LeptosOptions;
use openidconnect::{
    ClientId, ClientSecret, EndpointMaybeSet, EndpointNotSet, EndpointSet, IssuerUrl, RedirectUrl,
    core::{CoreClient, CoreProviderMetadata},
    reqwest::{Client, ClientBuilder, redirect},
};

pub type OidcClient = CoreClient<
    EndpointSet,      // HasAuthUrl
    EndpointNotSet,   // HasDeviceAuthUrl
    EndpointNotSet,   // HasIntrospectionUrl
    EndpointNotSet,   // HasRevocationUrl
    EndpointMaybeSet, // HasTokenUrl
    EndpointMaybeSet, // HasUserInfoUrl (adjust if you set it)
>;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub oauth_client: OidcClient,
    pub http_client: Client,
    pub jwt_encode: EncodingKey,
    pub jwt_decode: DecodingKey,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

impl AppState {
    pub async fn new(leptos_options: LeptosOptions) -> Self {
        let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID should be set");
        let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET should be set");

        let http_client = new_http_client();

        let issuer = IssuerUrl::new("https://accounts.google.com".to_string())
            .expect("hardcoded issuer URL should be valid");

        let provider_metadata = CoreProviderMetadata::discover_async(issuer, &http_client)
            .await
            .expect("Provider metadata should be discoverable");

        let site_addr = std::env::var("URL").expect("URL should be set");

        let oauth_client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
        )
        .set_redirect_uri(
            RedirectUrl::new(site_addr + "/auth/callback")
                .expect("Should be able to set redirect uri"),
        );

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET should be set");

        let jwt_encode = EncodingKey::from_secret(jwt_secret.as_ref());
        let jwt_decode = DecodingKey::from_secret(jwt_secret.as_ref());

        AppState {
            leptos_options,
            oauth_client,
            http_client,
            jwt_encode,
            jwt_decode,
        }
    }
}

fn new_http_client() -> Client {
    ClientBuilder::new()
        .redirect(redirect::Policy::none())
        .build()
        .expect("Reqwest client should build")
}
