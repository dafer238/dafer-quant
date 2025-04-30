// src/frontend/pages/home.rs
use leptos::*;
use crate::frontend::components::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="space-y-6">
            <h1 class="text-4xl font-bold">"Welcome to MyApp"</h1>
            <div class="space-y-4">
                <p>"Current count: " {count}</p>
                <button
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                    on:click=move |_| set_count.update(|n| *n += 1)
                >
                    "Increment"
                </button>
            </div>
        </div>
    }
}
