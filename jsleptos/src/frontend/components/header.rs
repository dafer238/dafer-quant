// src/frontend/components/header.rs
use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-white shadow">
            <nav class="container mx-auto px-4 py-6">
                <div class="flex justify-between items-center">
                    <A href="/" class="text-xl font-bold">MyApp</A>
                    <div class="space-x-4">
                        <A href="/" class="hover:text-gray-600">Home</A>
                        <A href="/about" class="hover:text-gray-600">About</A>
                    </div>
                </div>
            </nav>
        </header>
    }
}

