use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <span class="line">
            <p>"01"</p>
        </span>
        <section id="about">
            <div>
                <img src="portrait.avif" alt="Portrait of me." />
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
        <span class="line">
            <p>"02"</p>
        </span>
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
        <span class="line">
            <p>"03"</p>
        </span>
        <section id="irrel-ex">
            <div>
                <h2>"Irrelevant Experience"</h2>
                <p>
                    "If you made it this far and you’re still reading, I figure you may want to know more about me as a person."
                </p>
                <p>"This section will be filled in soon. Lorem ipsum dolor sit amet."</p>
            </div>
            <div>
                <img src="portrait.avif" alt="Portrait of me." />
            </div>
        </section>
    }
}
