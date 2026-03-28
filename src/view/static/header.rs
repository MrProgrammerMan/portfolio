use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <nav>
                <A href="/">"Home"</A>
                <A href="/about">"About"</A>
                <A href="/portfolio">"Portfolio"</A>
                <A href="/contact">"Contact"</A>
            </nav>
        </header>
    }
}
