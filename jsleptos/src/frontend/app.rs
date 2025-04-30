// src/frontend/app.rs
use crate::frontend::{
    components::{
        header::Header,
        //footer::Footer
    },
    pages::{about::AboutPage, home::HomePage},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="min-h-screen flex flex-col">
                <Header />
                <main class="flex-grow container mx-auto px-4 py-8">
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/about" view=AboutPage/>
                    </Routes>
                </main>
                //<Footer />
            </div>
        </Router>
    }
}
