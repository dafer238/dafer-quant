// ./jsl_frontend/src/main.rs

pub mod assets;
pub mod components;
pub mod pages;

use assets::*;
use pages::LandingPage;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Sets browser tab icon
        document::Link { rel: "icon", href: FAVICON }

        // Sets the main styesheets
        document::Link { rel: "stylesheet", href: VARIABLES_CSS }
        document::Link { rel: "stylesheet", href: HEADERS_CSS }
        document::Link { rel: "stylesheet", href: FOOTERS_CSS }
        document::Link { rel: "stylesheet", href: LINKS_CSS }
        document::Link { rel: "stylesheet", href: MENUS_CSS }
        document::Link { rel: "stylesheet", href: BUTTON_CSS }
        document::Link { rel: "stylesheet", href: CARD_CSS }
        document::Link { rel: "stylesheet", href: IMG_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        LandingPage {}

    }
}
