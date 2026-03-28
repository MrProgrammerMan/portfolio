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
    view! {
        <span class="line"><p>"01"</p></span>
        <section id="about">
        <div>
            <img src="portrait.jpg" alt="Portrait of me." />
        </div>
        <div>
            <h1>"About"</h1>
            <ul>
            <li>"Name: Jonas Hazeland Baugerud"</li>
            <li>"Student at Software Engineering, Oslomet"</li>
            <li>"Born: 03/07/2004"</li>
            <li>"Study location: Pilestredet, Oslo, Norway"</li>
            </ul>
        </div>
        </section>
        <span class="line"><p>"02"</p></span>
        <section class="full" id="rel-ex">
        <h2>"Relevant experience"</h2>
        <ul>
            <span class="arrow">
            <span></span>
            <span></span>
            <span></span>
            </span>
            <li>
            <p>"August 2024 - Today"</p>
            <p>"Software Engineering at OsloMet"</p>
            </li>
            <li>
            <p>"August 2023 - June 2024"</p>
            <p>"Software Engineering at NTNU"</p>
            </li>
            <li>
            <p>"August 2021 - June 2023"</p>
            <p>"Computer science 1 & 2 (upper secondary school) Focus on web and Python"</p>
            </li>
            <li>
            <p>"July 2004 - Today"</p>
            <p>"A love for technology and a passion for knowing more."</p>
            </li>
        </ul>
        </section>
        <span class="line"><p>"03"</p></span>
        <section id="irrel-ex">
        <div>
            <h2>"Irrelevant Experience"</h2>
            <p>"If you made it this far and you’re still reading, I figure you may want to know more about me as a person."</p>
            <p>"I grew up in a small town near Elverum."</p>
        </div>
        <div>
            <img src="portrait.jpg" alt="Portrait of me." />
        </div>
        </section>
    }
}

#[component]
fn Portfolio() -> impl IntoView {
    view! {
        <section class="full">
            <div>
                <h1>"Coming soon"</h1>
                <br/>
                <p>
                    "I'm working on it..."
                </p>
                <a href="https://github.com/MrProgrammerMan/portfolio">
                    "source"
                </a>
            </div>
        </section>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! { "The contact page" }
}
