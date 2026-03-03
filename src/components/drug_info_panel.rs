// Standalone drug info panel — actual component is in drug_selector.rs.

use crate::state::app_state::AppState;
use leptos::prelude::*;

#[component]
pub fn DrugInfoPanelStandalone() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");

    view! {
        {move || {
            let drug = state.selected_drug.get();
            match drug {
                None => view! { <div></div> }.into_any(),
                Some(d) => view! {
                    <div class="bg-white border border-gray-200 rounded-lg p-4 space-y-3">
                        <h3 class="font-semibold text-gray-800">{d.display_name()}</h3>
                        <div class="text-sm text-gray-600">
                            <p><strong>"หมวดหมู่: "</strong>{d.category.display_th().to_string()}</p>
                            <p><strong>"ช่องทาง: "</strong>{
                                d.age_dosings.first()
                                    .map(|ad| ad.route.display_th().to_string())
                                    .unwrap_or_else(|| "N/A".to_string())
                            }</p>
                        </div>
                        {(!d.formulations.is_empty()).then(|| view! {
                            <div>
                                <p class="text-xs font-medium text-gray-600">"Formulations:"</p>
                                <ul class="text-xs text-gray-500 mt-1">
                                    {d.formulations.iter().map(|f| {
                                        let desc = f.description.clone();
                                        view! { <li>{desc}</li> }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </div>
                        })}
                    </div>
                }.into_any(),
            }
        }}
    }
}
