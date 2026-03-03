use leptos::prelude::*;

#[allow(dead_code)]
#[component]
pub fn Tooltip(text: String, children: Children) -> impl IntoView {
    let show = RwSignal::new(false);
    let text_clone = text.clone();

    view! {
        <div class="relative inline-block">
            <div
                on:mouseenter=move |_| show.set(true)
                on:mouseleave=move |_| show.set(false)
                on:focus=move |_| show.set(true)
                on:blur=move |_| show.set(false)
            >
                {children()}
            </div>
            {move || show.get().then(|| {
                let t = text_clone.clone();
                view! {
                    <div
                        class="absolute z-50 bottom-full left-1/2 -translate-x-1/2 mb-2
                               bg-gray-800 text-white text-xs rounded-lg px-3 py-2
                               whitespace-nowrap shadow-lg"
                        role="tooltip"
                    >
                        {t}
                        <div class="absolute top-full left-1/2 -translate-x-1/2 border-4 border-transparent border-t-gray-800"></div>
                    </div>
                }
            })}
        </div>
    }
}
