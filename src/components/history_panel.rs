use crate::state::app_state::AppState;
use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn HistoryPanel() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");

    let confirm_clear = RwSignal::new(false);

    view! {
        <div class="history-panel bg-white rounded-xl shadow-sm border border-gray-200 p-4">
            <div class="flex items-center justify-between mb-3">
                <h2 class="text-base font-semibold text-gray-800">"ประวัติการคำนวณ"</h2>
                // Clear button
                {move || {
                    let history = state.history.get();
                    if history.is_empty() {
                        return Either::Left(());
                    }
                    if confirm_clear.get() {
                        Either::Right(Either::Left(view! {
                            <div class="flex gap-2">
                                <button
                                    class="text-xs text-red-600 hover:text-red-800"
                                    on:click=move |_| {
                                        state.clear_history();
                                        confirm_clear.set(false);
                                    }
                                >
                                    "ยืนยันลบทั้งหมด"
                                </button>
                                <button
                                    class="text-xs text-gray-500 hover:text-gray-700"
                                    on:click=move |_| confirm_clear.set(false)
                                >
                                    "ยกเลิก"
                                </button>
                            </div>
                        }))
                    } else {
                        Either::Right(Either::Right(view! {
                            <button
                                class="text-xs text-gray-500 hover:text-red-600 transition-colors"
                                on:click=move |_| confirm_clear.set(true)
                            >
                                "🗑 ลบทั้งหมด"
                            </button>
                        }))
                    }
                }}
            </div>

            // History entries
            {move || {
                let history = state.history.get();
                if history.is_empty() {
                    return Either::Left(view! {
                        <p class="text-gray-400 text-sm italic text-center py-4">"ยังไม่มีประวัติการคำนวณ"</p>
                    });
                }

                Either::Right(view! {
                    <ul class="space-y-2 max-h-64 overflow-y-auto">
                        {history.into_iter().map(|entry| {
                            let entry_id = entry.id.clone();
                            let drug_name = entry.drug_display_name.clone();
                            let detail = format!(
                                "{} × {} | {} | {}",
                                entry.single_dose_mg, entry.frequency,
                                entry.patient_weight_kg, entry.age_group,
                            );
                            let critical = entry.had_critical_warning;

                            view! {
                                <li class="flex items-center justify-between p-2 rounded-lg hover:bg-gray-50 text-sm group">
                                    <div class="flex-1 min-w-0">
                                        <p class="font-medium text-gray-800 truncate">{drug_name}</p>
                                        <p class="text-xs text-gray-500">{detail}</p>
                                    </div>
                                    {critical.then(|| view! {
                                        <span class="text-red-500 text-xs mr-2">"⚠"</span>
                                    })}
                                    <button
                                        class="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-500 transition-opacity text-xs"
                                        aria-label="ลบรายการนี้"
                                        on:click=move |_| {
                                            state.remove_history_entry(&entry_id);
                                        }
                                    >
                                        "✕"
                                    </button>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                })
            }}
        </div>
    }
}
