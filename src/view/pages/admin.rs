use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    any_nested_route::IntoAnyNestedRoute,
    components::{Outlet, Route},
    path,
};
use overview::Overview;

mod overview;

#[component(transparent)]
pub fn AdminRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <Route path=path!("") view=Overview />
        <Route path=path!("/overview") view=Overview />
    }
    .into_inner()
    .into_any_nested_route()
}

#[component()]
pub fn Admin() -> impl IntoView {
    view! { <Outlet /> }
}
