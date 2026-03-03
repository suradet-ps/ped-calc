use leptos::prelude::*;

#[allow(dead_code)]
#[component]
pub fn InputField(
    label: String,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] unit: Option<String>,
) -> impl IntoView {
    let is_required = required.unwrap_or(false);
    let has_error = error.is_some();
    let error_text = error.clone();

    view! {
        <div class="form-group">
            <label class="block text-sm font-medium text-gray-700 mb-1">
                {label.clone()}
                {is_required.then(|| view! { <span class="text-red-500 ml-0.5">"*"</span> })}
            </label>
            <div class="relative">
                <input
                    type="text"
                    placeholder=placeholder.unwrap_or_default()
                    aria-label=label
                    class=if has_error {
                        "w-full border border-red-400 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-red-300"
                    } else {
                        "w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-300"
                    }
                />
                {unit.map(|u| view! {
                    <span class="absolute right-3 top-2.5 text-gray-400 text-sm">{u}</span>
                })}
            </div>
            {error_text.map(|e| view! {
                <p class="text-red-600 text-xs mt-1" role="alert">{e}</p>
            })}
        </div>
    }
}
