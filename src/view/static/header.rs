use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <Logo />
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

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <svg viewBox="0 -10 227 100" xmlns="http://www.w3.org/2000/svg">
            <circle cx="22" cy="45" r="10" fill="#7888FF" opacity="0.5" />
            <path
                d="M49,25 L83,45 L49,65"
                fill="none"
                stroke="#7888FF"
                stroke-width="4"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            <path
                d="M93,25 L127,45 L93,65"
                fill="none"
                stroke="#7888FF"
                stroke-width="4"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            <line
                x1="137"
                y1="35"
                x2="171"
                y2="35"
                stroke="#7888FF"
                stroke-width="4"
                stroke-linecap="round"
            />
            <line
                x1="137"
                y1="55"
                x2="171"
                y2="55"
                stroke="#7888FF"
                stroke-width="4"
                stroke-linecap="round"
            />
            <line
                x1="98"
                y1="17"
                x2="93"
                y2="3"
                stroke="#F74C00"
                stroke-width="2"
                stroke-linecap="round"
                opacity="1"
            />
            <line
                x1="110"
                y1="15"
                x2="110"
                y2="0"
                stroke="#F74C00"
                stroke-width="2"
                stroke-linecap="round"
                opacity="1"
            />
            <line
                x1="122"
                y1="17"
                x2="127"
                y2="3"
                stroke="#F74C00"
                stroke-width="2"
                stroke-linecap="round"
                opacity="1"
            />
            <circle cx="198" cy="45" r="17" fill="none" stroke="#16BBA0" stroke-width="2.5" />
            <circle
                cx="198"
                cy="45"
                r="11"
                fill="none"
                stroke="#16BBA0"
                stroke-width="1.5"
                opacity="0.5"
            />
            <circle cx="198" cy="45" r="5" fill="#16BBA0" />
        </svg>
    }
}
