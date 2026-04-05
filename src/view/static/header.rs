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
        <svg
            viewBox="0 -10 227 100"
            version="1.1"
            id="svg8"
            sodipodi:docname="personal_logo.svg"
            inkscape:version="1.4.3 (0d15f75042, 2025-12-25)"
            xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
            xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:svg="http://www.w3.org/2000/svg"
        >
            <defs id="defs8" />
            <sodipodi:namedview
                id="namedview8"
                pagecolor="#ffffff"
                bordercolor="#000000"
                borderopacity="0.25"
                inkscape:showpageshadow="2"
                inkscape:pageopacity="0.0"
                inkscape:pagecheckerboard="0"
                inkscape:deskcolor="#d1d1d1"
                inkscape:zoom="4.8502203"
                inkscape:cx="113.39691"
                inkscape:cy="49.997729"
                inkscape:window-width="2560"
                inkscape:window-height="1374"
                inkscape:window-x="0"
                inkscape:window-y="0"
                inkscape:window-maximized="1"
                inkscape:current-layer="svg8"
            />
            <circle cx="22" cy="45" r="10" fill="#7888FF" opacity="0.5" id="circle1" />
            <path
                d="M49,25 L83,45 L49,65"
                fill="none"
                stroke="#7888FF"
                stroke-width="4"
                stroke-linecap="round"
                stroke-linejoin="round"
                id="path1"
                style="stroke-width:6;stroke-dasharray:none"
            />
            <path
                d="M93,25 L127,45 L93,65"
                fill="none"
                stroke="#7888FF"
                stroke-width="4"
                stroke-linecap="round"
                stroke-linejoin="round"
                id="path2"
                style="stroke-width:6;stroke-dasharray:none"
            />
            <line
                x1="137"
                y1="35"
                x2="171"
                y2="35"
                stroke="#7888FF"
                stroke-width="4"
                stroke-linecap="round"
                id="line2"
                style="stroke-width:6;stroke-dasharray:none"
            />
            <line
                x1="137"
                y1="55"
                x2="171"
                y2="55"
                stroke="#7888FF"
                stroke-width="4"
                stroke-linecap="round"
                id="line3"
                style="stroke-width:6;stroke-dasharray:none"
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
                id="line4"
                style="stroke-width:3;stroke-dasharray:none"
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
                id="line5"
                style="stroke-width:3;stroke-dasharray:none"
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
                id="line6"
                style="stroke-width:3;stroke-dasharray:none"
            />
            <circle
                cx="198"
                cy="45"
                r="17"
                fill="none"
                stroke="#16BBA0"
                stroke-width="2.5"
                id="circle6"
            />
            <circle
                cx="198"
                cy="45"
                r="11"
                fill="none"
                stroke="#16BBA0"
                stroke-width="1.5"
                opacity="0.5"
                id="circle7"
            />
            <circle cx="198" cy="45" r="5" fill="#16BBA0" id="circle8" />
        </svg>
    }
}
