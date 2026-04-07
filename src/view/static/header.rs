use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <img src="personal_logo.svg" />
            <label for="menu-btn" class="menu-icon">
                <span></span>
                <span></span>
                <span></span>
            </label>
            <input type="checkbox" id="menu-btn" hidden />
            <p>"Jonas_Baugerud@OsloMet:~$"</p>
            <nav>
                <A href="/">"Home"</A>
                <A href="/about">"About"</A>
                <A href="/portfolio">"Portfolio"</A>
                <A href="/contact">"Contact"</A>
            </nav>
        </header>
    }
}