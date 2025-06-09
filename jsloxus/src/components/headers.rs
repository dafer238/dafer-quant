// ./jsloxus/src/components/headers.rs

use crate::assets::DARK_THEME;
use crate::assets::JSL_LOGO;
use dioxus::prelude::*;
// use crate::assets::HEADER_LOGO;

#[component]
pub fn HeaderMenu() -> Element {
    rsx! {
        div {
            id: "main_menu",
            div {
                id: "left_menu",
                a { href: "", img { src: JSL_LOGO, id: "header_logo", alt: "Logo" }}
                a { href: "", "Home"}
                a { href: "", "Features"}
                a { href: "", "About Us"}
                a { href: "", "Contact"}
            }
            div {
                id: "right_menu",
                img { src: DARK_THEME, id: "theme_icon", alt: "Theme" }
                a { href: "", "EN 🇺🇸"}
                a { href: "", "Log In"}
                a { href: "", "Sign Up"}
            }
        }
    }
}
