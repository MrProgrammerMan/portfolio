use leptos::prelude::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <h1>"The login page"</h1>
        <a href="/auth/login" rel="external">
            Log in with google
        </a>
    }
}
