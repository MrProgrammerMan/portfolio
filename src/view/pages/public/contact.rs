use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section>
            <div>
                <img src="portrait.avif" alt="Portrait of me." />
            </div>
            <div>
                <h1>"Contact"</h1>
                <ul>
                    <li>"Name: Jonas Hazeland Baugerud"</li>
                    <li>
                        "E-mail: "
                        <a href="mailto:jonas.baugerud@gmail.com">"jonas.baugerud@gmail.com"</a>
                    </li>
                    <li>
                        "Ditio e-mail: "<a href="mailto:bedrift@ditio.org">"bedrift@ditio.org"</a>
                    </li>
                    <li>
                        "School e-mail: "
                        <a href="mailto:jobau8311@oslomet.no">"jobau8311@oslomet.no"</a>
                    </li>
                    <li>
                        "You can also reach out to me through my LinkedIn profile, found "
                        <a
                            href="https://www.linkedin.com/in/jonas-baugerud/"
                            aria-label="My linkedin profile"
                        >
                            "here"
                        </a>" or at the bottom of the page."
                    </li>
                </ul>
            </div>
        </section>
    }
}
