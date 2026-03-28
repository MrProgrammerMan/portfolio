use leptos::prelude::*;
use leptos_router::{components::{Route, Router, Routes, A}, path};
use r#static::header::Header;

mod r#static;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header/>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/about") view=About/>
                    <Route path=path!("/portfolio") view=Portfolio/>
                    <Route path=path!("/contact") view=Contact/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {"The home page"}
}

#[component]
fn About() -> impl IntoView {
    view! {"The about page"}
}

#[component]
fn Portfolio() -> impl IntoView {
    view! {"The portfolio page"}
}

#[component]
fn Contact() -> impl IntoView {
    view! {"The contact page"}
}