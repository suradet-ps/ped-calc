use crate::state::app_state::AppState;
use crate::types::calculation::{DoseWarning, WarningSeverity};
use leptos::prelude::*;

#[component]
pub fn WarningBanner(warnings: Vec<DoseWarning>) -> impl IntoView {
    if warnings.is_empty() {
        return view! { <div></div> }.into_any();
    }

    let state = use_context::<AppState>().expect("AppState must be provided");

    let has_critical = warnings
        .iter()
        .any(|w| matches!(w.severity(), WarningSeverity::Critical));

    view! {
        <div class="warning-banner p-4 space-y-2">
            {warnings.into_iter().map(|warning| {
                let severity = warning.severity();
                let color_class = severity.color_class().to_string();
                let icon = severity.icon().to_string();
                let label = severity.label_th().to_string();
                let message = warning.message_th();
                let needs_alert = matches!(severity, WarningSeverity::Critical | WarningSeverity::Warning);

                view! {
                    <div
                        class=format!("rounded-lg border-l-4 p-3 {}", color_class)
                        role=if needs_alert { "alert" } else { "note" }
                    >
                        <div class="flex items-start gap-2">
                            <span class="text-lg" aria-hidden="true">{icon}</span>
                            <div class="flex-1">
                                <span class="font-semibold text-sm">{label}": "</span>
                                <span class="text-sm">{message}</span>
                            </div>
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}

            // Confirmation checkbox for critical warnings
            {has_critical.then(move || view! {
                <div class="mt-3 p-3 bg-red-50 rounded-lg border border-red-300">
                    <label class="flex items-start gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            class="mt-0.5 accent-red-600"
                            aria-label="ยืนยันว่าได้อ่านคำเตือนแล้ว"
                            prop:checked=move || state.user_confirmed_warning.get()
                            on:change=move |ev: web_sys::Event| {
                                let checked = ev.target()
                                    .and_then(|t| {
                                        use wasm_bindgen::JsCast;
                                        t.dyn_into::<web_sys::HtmlInputElement>().ok()
                                    })
                                    .map(|el| el.checked())
                                    .unwrap_or(false);
                                state.user_confirmed_warning.set(checked);
                            }
                        />
                        <span class="text-sm text-red-800 font-medium">
                            "ข้าพเจ้าได้อ่านคำเตือนทั้งหมดแล้ว และยืนยันว่าจะใช้ขนาดยานี้ภายใต้การดูแลทางการแพทย์"
                        </span>
                    </label>
                </div>
            })}
        </div>
    }.into_any()
}
