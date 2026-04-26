use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    any_nested_route::IntoAnyNestedRoute,
    components::{Outlet, Route},
    path,
};

use crate::{
    pages::public::{about::About, contact::Contact, home::Home, portfolio::Portfolio},
    r#static::{footer::Footer, header::Header},
};

mod about;
mod contact;
mod home;
mod portfolio;

#[component(transparent)]
pub fn PublicRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <Route path=path!("/") view=Home />
        <Route path=path!("/about") view=About />
        <Route path=path!("/portfolio") view=Portfolio />
        <Route path=path!("/contact") view=Contact />
    }
    .into_inner()
    .into_any_nested_route()
}

#[component]
pub fn Public() -> impl IntoView {
    view! {
        <Header />
        <main>
            <Outlet />
        </main>
        <Footer />
    }
}
