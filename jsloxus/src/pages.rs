// ./jsloxus/src/pages/landingpage.rs

use dioxus::prelude::*;

use crate::assets::PFP_GORKA;
use crate::assets::PFP_KEPA;

use crate::components::footers::Footer;
use crate::components::headers::HeaderMenu;

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        HeaderMenu {}
        // TestLinks {}
        LandingContent {}
        Footer {}
    }
}

#[component]
pub fn LandingContent() -> Element {
    rsx! {
        div {id: "founders",
            div {id:"founder",
                a {href: "", img {src: PFP_KEPA, id: "pfp", alt: "Kepa"}}
                p { "Joselito Joselitkinson" }
            }
            div {id: "founder",
                a {href: "", img {src: PFP_GORKA, id: "pfp", alt: "Kepa"}}
                p { "Moro Joselitkinson"}
            }
        }
    }
}
