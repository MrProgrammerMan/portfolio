use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};
use r#static::{footer::Footer, header::Header};

mod r#static;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About />
                    <Route path=path!("/portfolio") view=Portfolio />
                    <Route path=path!("/contact") view=Contact />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <section class="reverse">
            <div>
                <img src="portrait.jpg" alt="Portrait of me, Jonas Hazeland Baugerud." />
            </div>
            <div>
                <h1>"Hello, I'm Jonas"</h1>
                <p>
                    "I study "
                    <a href="https://www.oslomet.no/studier/tkd/dataingenior">
                        "Software Engineering at OsloMet"
                    </a>
                    ", a course focusing on programming, mathematics, design philosophy and cooperation. Currently, I study full time, but I am always interested in job opportunities to the side."
                </p>
                <p>
                    "This is my portfolio. Here you will be able to find projects I have completed in order to see how I work(once I finish "
                    <a href="https://github.com/MrProgrammerMan/portfolio">"the site"</a>
                    "). You can also find information about me as a person, and get in touch with me if you want. I recommend you start in the "
                    <A href="portfolio">"portfolio"</A>"."
                </p>
            </div>
        </section>
    }
}

#[component]
fn About() -> impl IntoView {
    view! { "The about page" }
}

#[component]
fn Portfolio() -> impl IntoView {
    view! { "The portfolio page" }
}

#[component]
fn Contact() -> impl IntoView {
    view! { "The contact page" }
}
