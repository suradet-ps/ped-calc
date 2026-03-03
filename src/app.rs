use crate::state::app_state::AppState;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    let app_state = AppState::new();
    provide_context(app_state);

    view! {
        <Router>
            <div class="min-h-screen bg-gray-50">
                <crate::components::layout::navbar::Navbar />

                <main class="max-w-6xl mx-auto px-4 py-6">
                    <Routes fallback=|| view! { <p>"ไม่พบหน้าที่ต้องการ"</p> }>
                        <Route path=path!("/") view=crate::pages::calculator::CalculatorPage />
                        <Route path=path!("/reference") view=crate::pages::drug_reference::DrugReferencePage />
                        <Route path=path!("/about") view=crate::pages::about::AboutPage />
                    </Routes>
                </main>

                <crate::components::layout::footer::Footer />
            </div>
        </Router>
    }
}
