// src/frontend/pages/about.rs
use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-4xl font-bold">"About Us"</h1>
            <p class="text-lg">"This is a Rust-powered web application using Axum and Leptos."</p>
        </div>
    }
}
