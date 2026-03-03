use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="bg-white border-b border-gray-200 sticky top-0 z-50">
            <div class="max-w-6xl mx-auto px-4 h-14 flex items-center justify-between">
                <A href="/" attr:class="flex items-center gap-2 no-underline">
                    <span class="text-blue-600 font-bold text-lg">"PedCalc"</span>
                    <span class="text-gray-400 text-sm">"คำนวณขนาดยาเด็ก"</span>
                </A>
                <div class="flex gap-4 text-sm">
                    <A href="/" attr:class="text-gray-600 hover:text-blue-600 transition-colors no-underline">"คำนวณ"</A>
                    <A href="/reference" attr:class="text-gray-600 hover:text-blue-600 transition-colors no-underline">"ตำราอ้างอิง"</A>
                    <A href="/about" attr:class="text-gray-600 hover:text-blue-600 transition-colors no-underline">"เกี่ยวกับ"</A>
                </div>
            </div>
        </nav>
    }
}
