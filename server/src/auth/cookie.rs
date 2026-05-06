use leptos_use::SameSite;
use tower_sessions::cookie::{Cookie, time::Duration};

pub struct CookieBuilder;

impl CookieBuilder {
    pub fn jwt(token: String) -> Cookie<'static> {
        Cookie::build(("jwt", token))
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(Duration::minutes(5))
            .build()
    }
    pub fn refresh(token: String) -> Cookie<'static> {
        Cookie::build(("refresh", token))
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(Duration::days(30))
            .build()
    }
}