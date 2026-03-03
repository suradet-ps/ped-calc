use leptos::prelude::*;

#[allow(dead_code)]
#[component]
pub fn SelectField(
    label: String,
    options: Vec<(String, String)>,
    #[prop(optional)] error: Option<String>,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label class="block text-sm font-medium text-gray-700 mb-1">{label.clone()}</label>
            <select
                class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-300"
                aria-label=label
            >
                {options.into_iter().map(|(value, label)| view! {
                    <option value=value>{label}</option>
                }).collect::<Vec<_>>()}
            </select>
            {error.map(|e| view! {
                <p class="text-red-600 text-xs mt-1" role="alert">{e}</p>
            })}
        </div>
    }
}
