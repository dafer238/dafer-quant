// ./jsloxus/src/main.rs

pub mod assets;
pub mod components;

use assets::FAVICON;
use assets::MAIN_CSS;
use components::footers::Footer;
use components::headers::HeaderMenu;
use components::modules::TestLinks;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Sets browser tab icon
        document::Link { rel: "icon", href: FAVICON }
        // Sets the main styesheet
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        LandingPage {}

    }
}

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        HeaderMenu {}
        TestLinks {}
        Footer {}
    }
}
