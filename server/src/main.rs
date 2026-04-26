use crate::auth::auth_login_handler;

pub mod auth;
pub mod state;
pub mod error;

#[tokio::main]
async fn main() {
    use app::{App, shell};
    use auth::auth_callback_handler;
    use axum::{Router, routing::get};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use state::AppState;
    use tower_sessions::{MemoryStore, SessionManagerLayer};

    dotenvy::dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let state = AppState::new(leptos_options).await;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(leptos_use::SameSite::Lax);

    let app = Router::new()
        .route("/auth/login", get(auth_login_handler))
        .route("/auth/callback", get(auth_callback_handler))
        .leptos_routes(&state, routes, {
            let leptos_options = state.leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(session_layer)
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
