use dioxus::{logger::tracing, prelude::*};


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "app-container",

            header { class: "navbar",
                div { class: "logo", "PSJOB" }
                button { class: "menu-button", "▤" }
            }
        }
        Hero {}
        Cta {}
        footer { class: "footer", "© 2026 PSJOB. Find your Hustle." }

    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero-content",
                span { class: "badge", "Making a living" }
                h1 { class: "highlight", "Find your job" }
            }
        }
    }
}
#[component]
fn Cta() -> Element {
    rsx! {
        section { class: "cta-section",

            h1 { "Ready to get started?" }
            div {
                button { class: "signup-button", "Sign Up ➡️" }
            }
        }
    }
}



