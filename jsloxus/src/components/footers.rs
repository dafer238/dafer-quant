// ./jsloxus/src/components/footers.rs

use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            id: "footer",
            div {
                id: "footer_content",
                p { "Contact us:" }
                p { "Email: " a{ href: "mailto:jslfunds@gmail.com", "jslfunds@gmail.com" }}
                p { "Phone: " a{ href: "tel:+1 (123) 456-7890", "+1 (123) 456-7890" }}
                p { "Address: JSL street, JSLitkinson" }
            }
        }
    }
}
