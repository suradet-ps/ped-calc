use crate::data::drugs::all_drugs;
use crate::state::app_state::AppState;
use leptos::prelude::*;

#[component]
pub fn DrugSelector() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");

    let search_text = RwSignal::new(String::new());
    let highlight_index = RwSignal::new(0i32);
    let show_list = RwSignal::new(true);

    let all = all_drugs();
    let drugs_data = StoredValue::new(all);

    // Filtered drugs memo
    let filtered_drugs = Memo::new(move |_| {
        let q = search_text.get();
        let drugs = drugs_data.get_value();
        if q.is_empty() {
            drugs
        } else {
            drugs
                .into_iter()
                .filter(|d| d.matches_search(&q))
                .collect::<Vec<_>>()
        }
    });

    // Keyboard handler
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        let count = filtered_drugs.get().len() as i32;
        match key.as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                highlight_index.update(|i| *i = (*i + 1).min(count - 1));
            }
            "ArrowUp" => {
                ev.prevent_default();
                highlight_index.update(|i| *i = (*i - 1).max(0));
            }
            "Enter" => {
                ev.prevent_default();
                let idx = highlight_index.get() as usize;
                let drugs = filtered_drugs.get();
                if let Some(drug) = drugs.get(idx) {
                    state.selected_drug.set(Some(drug.clone()));
                    show_list.set(false);
                }
            }
            "Escape" => {
                show_list.set(false);
            }
            _ => {}
        }
    };

    view! {
        <div class="drug-selector">
            <label class="block text-sm font-medium text-gray-700 mb-1">
                "เลือกยา "
                <span class="text-red-500">"*"</span>
            </label>

            // Search input
            <input
                type="search"
                class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-300"
                placeholder="พิมพ์ชื่อยา เช่น Amoxicillin, PCM, Augmentin..."
                aria-label="ค้นหายา"
                prop:value=move || search_text.get()
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    search_text.set(val.clone());
                    state.search_query.set(val);
                    highlight_index.set(0);
                    show_list.set(true);
                }
                on:focus=move |_| show_list.set(true)
                on:keydown=on_keydown
            />

            // Selected drug display
            {move || {
                let drug = state.selected_drug.get();
                match drug {
                    Some(d) => view! {
                        <div class="mt-2 p-3 bg-blue-50 border-l-4 border-blue-600 rounded-r-lg">
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="font-semibold text-gray-900">{d.generic_name.clone()}</span>
                                    <span class="text-sm text-gray-500 ml-2">
                                        {d.brand_names.join(", ")}
                                    </span>
                                </div>
                                <button
                                    class="text-gray-400 hover:text-red-500 text-lg"
                                    aria-label="ยกเลิกการเลือกยา"
                                    on:click=move |_| {
                                        state.selected_drug.set(None);
                                        search_text.set(String::new());
                                        show_list.set(true);
                                    }
                                >
                                    "✕"
                                </button>
                            </div>
                            <span class="inline-block mt-1 text-xs rounded-full px-2 py-0.5 bg-blue-100 text-blue-700">
                                {d.category.display_en().to_string()}
                            </span>
                        </div>
                    }.into_any(),
                    None => view! { <div></div> }.into_any(),
                }
            }}

            // Drug list
            {move || {
                if !show_list.get() || state.selected_drug.get().is_some() {
                    return view! { <div></div> }.into_any();
                }
                let drugs = filtered_drugs.get();
                let hi = highlight_index.get();

                if drugs.is_empty() {
                    return view! {
                        <div class="mt-2 text-sm text-gray-500 italic p-3 bg-gray-50 rounded-lg">
                            "ไม่พบยาที่ตรงกับการค้นหา"
                        </div>
                    }.into_any();
                }

                view! {
                    <ul class="mt-2 border border-gray-200 rounded-lg max-h-64 overflow-y-auto divide-y divide-gray-100" role="listbox">
                        {drugs.into_iter().enumerate().map(|(idx, drug)| {
                            let drug_clone = drug.clone();
                            let is_highlighted = idx as i32 == hi;
                            let bg = if is_highlighted { "bg-blue-50" } else { "bg-white hover:bg-gray-50" };
                            view! {
                                <li
                                    class=format!("px-3 py-3 cursor-pointer transition-colors min-h-[56px] flex items-center justify-between {bg}")
                                    role="option"
                                    aria-selected=is_highlighted
                                    on:click=move |_| {
                                        state.selected_drug.set(Some(drug_clone.clone()));
                                        show_list.set(false);
                                    }
                                >
                                    <div>
                                        <span class="font-medium text-gray-900">{drug.generic_name.clone()}</span>
                                        <span class="text-xs text-gray-500 ml-2">
                                            {drug.brand_names.join(", ")}
                                        </span>
                                    </div>
                                    <span class="text-xs rounded-full px-2 py-0.5 bg-gray-100 text-gray-600">
                                        {drug.category.display_en().to_string()}
                                    </span>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                }.into_any()
            }}

            // Drug Info Panel (when selected)
            <DrugInfoPanel />
        </div>
    }
}

/// Panel displaying details of the selected drug.
#[component]
pub fn DrugInfoPanel() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");

    view! {
        {move || {
            let drug = state.selected_drug.get();
            match drug {
                None => view! { <div></div> }.into_any(),
                Some(d) => view! {
                    <div class="mt-3 bg-white border border-gray-200 rounded-lg p-4 space-y-3">
                        <h3 class="text-sm font-semibold text-gray-700">"ข้อมูลยา"</h3>

                        // Contraindications
                        {if !d.contraindications.is_empty() {
                            view! {
                                <div>
                                    <p class="text-xs font-medium text-red-700">"ข้อห้ามใช้:"</p>
                                    <ul class="list-disc list-inside text-xs text-red-600 mt-1">
                                        {d.contraindications.iter().map(|c| view! {
                                            <li>{c.clone()}</li>
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }}

                        // Details
                        <div class="grid grid-cols-2 gap-2 text-xs">
                            <div>
                                <span class="text-gray-500">"Pregnancy: "</span>
                                <span class="font-medium">{d.pregnancy_category.clone().unwrap_or_else(|| "N/A".into())}</span>
                            </div>
                            <div>
                                <span class="text-gray-500">"Renal adj: "</span>
                                <span class="font-medium">{if d.requires_renal_adjustment { "Yes" } else { "No" }}</span>
                            </div>
                            <div>
                                <span class="text-gray-500">"Hepatic adj: "</span>
                                <span class="font-medium">{if d.requires_hepatic_adjustment { "Yes" } else { "No" }}</span>
                            </div>
                            <div>
                                <span class="text-gray-500">"Last reviewed: "</span>
                                <span class="font-medium">{d.last_reviewed.clone()}</span>
                            </div>
                        </div>

                        <p class="text-xs text-gray-400">{format!("อ้างอิง: {}", d.reference)}</p>
                    </div>
                }.into_any(),
            }
        }}
    }
}
