// ./pmm-frontend/src/components/modules.rs

use dioxus::prelude::*;

#[component]
pub fn TestLinks() -> Element {
    rsx! {
        div {
            id: "links",
            a { href: "https://dioxuslabs.com/learn/0.6/", "Learn Dioxus" }
            a { href: "https://dioxuslabs.com/awesome", "Awesome Dioxus" }
        }
    }
}
