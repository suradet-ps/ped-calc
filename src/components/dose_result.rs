use crate::components::warning_banner::WarningBanner;
use crate::state::app_state::{AppState, HistoryEntry};
use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn DoseResult() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");

    let copy_feedback = RwSignal::new(false);

    view! {
        {move || {
            let result = state.dose_result.get();
            match result {
                None => Either::Left(()),

                Some(Err(e)) => Either::Right(Either::Left(view! {
                    <div class="bg-red-50 border border-red-300 rounded-lg p-4">
                        <p class="text-red-700 font-medium">"เกิดข้อผิดพลาด"</p>
                        <p class="text-red-600 text-sm mt-1">{e.to_string()}</p>
                    </div>
                })),

                Some(Ok(r)) => {
                    let warnings = r.warnings.clone();
                    let has_warnings = !warnings.is_empty();
                    let ml_string = r.display_dose.ml_string.clone();
                    let formulation = r.display_dose.formulation_used.clone();
                    let mg_string = r.display_dose.mg_string.clone();
                    let min_dose = r.min_single_dose_mg;
                    let max_dose = r.max_single_dose_mg;
                    let daily = r.daily_dose_mg;
                    let freq = r.frequency.display_th();
                    let weight = r.weight_used_kg;
                    let age_group = r.age_group_used.display_th().to_string();
                    let dpk = r.dose_per_kg;
                    let has_critical = r.has_critical_warning();
                    let freq_abbrev = r.frequency.abbreviation();
                    let drug_ref = state.selected_drug.get_untracked().map(|d| d.reference.clone()).unwrap_or_default();

                    Either::Right(Either::Right(view! {
                        <div class="dose-result-card bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
                            // Main Dose Display
                            <div class="p-6 bg-gradient-to-br from-blue-50 to-white">
                                <p class="text-sm text-gray-500 mb-1">"ขนาดยาต่อครั้ง"</p>
                                <div class="flex items-baseline gap-2">
                                    <span class="text-4xl font-bold text-gray-900">
                                        {mg_string.clone()}
                                    </span>
                                </div>

                                {ml_string.map(|ml| view! {
                                    <p class="text-blue-600 mt-1 font-medium">{format!("= {}", ml)}</p>
                                })}

                                {formulation.map(|f| view! {
                                    <p class="text-xs text-gray-400 mt-0.5">{format!("({})", f)}</p>
                                })}

                                <p class="text-sm text-gray-500 mt-2">
                                    {format!("ช่วงแนะนำ: {:.0}–{:.0} mg", min_dose, max_dose)}
                                </p>
                            </div>

                            // Details section
                            <div class="px-6 py-4 border-t border-gray-100 space-y-2">
                                <div class="flex justify-between text-sm">
                                    <span class="text-gray-500">"ขนาดรวมต่อวัน"</span>
                                    <span class="font-medium">{format!("{:.0} mg/day", daily)}</span>
                                </div>
                                <div class="flex justify-between text-sm">
                                    <span class="text-gray-500">"ความถี่"</span>
                                    <span class="font-medium">{freq}</span>
                                </div>
                                <div class="flex justify-between text-sm">
                                    <span class="text-gray-500">"น้ำหนักที่ใช้คำนวณ"</span>
                                    <span class="font-medium">{format!("{:.1} kg", weight)}</span>
                                </div>
                                <div class="flex justify-between text-sm">
                                    <span class="text-gray-500">"กลุ่มอายุ"</span>
                                    <span class="font-medium">{age_group}</span>
                                </div>
                                {(dpk > rust_decimal::Decimal::ZERO).then(|| view! {
                                    <div class="flex justify-between text-sm">
                                        <span class="text-gray-500">"Dose/kg"</span>
                                        <span class="font-medium">{format!("{} mg/kg/dose", dpk)}</span>
                                    </div>
                                })}
                            </div>

                            // Warnings
                            {has_warnings.then(|| view! {
                                <div class="border-t border-gray-100">
                                    <WarningBanner warnings=warnings />
                                </div>
                            })}

                            // Actions
                            <div class="px-6 py-4 border-t border-gray-100 flex gap-3">
                                <button
                                    class="flex-1 bg-blue-600 text-white rounded-lg py-2 text-sm font-medium hover:bg-blue-700 transition-colors"
                                    on:click={
                                        let mg_str = mg_string.clone();
                                        let fa = freq_abbrev.clone();
                                        move |_: web_sys::MouseEvent| {
                                            let text = format!("{} ({})", mg_str, fa);
                                            if let Some(window) = web_sys::window() {
                                                let clipboard = window.navigator().clipboard();
                                                let _ = clipboard.write_text(&text);
                                                copy_feedback.set(true);
                                                set_timeout(
                                                    move || copy_feedback.set(false),
                                                    std::time::Duration::from_secs(2),
                                                );
                                            }
                                        }
                                    }
                                >
                                    {move || if copy_feedback.get() { "✅ คัดลอกแล้ว!" } else { "📋 คัดลอก" }}
                                </button>
                                <button
                                    class="flex-1 border border-gray-300 text-gray-700 rounded-lg py-2 text-sm font-medium hover:bg-gray-50 transition-colors"
                                    on:click={
                                        let mg_str2 = mg_string.clone();
                                        let fa2 = freq_abbrev.clone();
                                        let ag2 = r.age_group_used.display_th().to_string();
                                        let drug_name = state.selected_drug.get_untracked().map(|d| d.display_name()).unwrap_or_default();
                                        let drug_id = state.selected_drug.get_untracked().map(|d| d.id.clone()).unwrap_or_default();
                                        move |_: web_sys::MouseEvent| {
                                            let entry = HistoryEntry {
                                                id: format!("{}", js_sys::Date::now() as u64),
                                                timestamp: js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default(),
                                                drug_id: drug_id.clone(),
                                                drug_display_name: drug_name.clone(),
                                                patient_weight_kg: format!("{}", weight),
                                                single_dose_mg: mg_str2.clone(),
                                                frequency: fa2.clone(),
                                                age_group: ag2.clone(),
                                                had_critical_warning: has_critical,
                                            };
                                            state.add_to_history(entry);
                                        }
                                    }
                                >
                                    "💾 บันทึก"
                                </button>
                            </div>

                            // Reference
                            <div class="px-6 py-3 bg-gray-50 text-xs text-gray-400">
                                {format!("อ้างอิง: {}", drug_ref)}
                            </div>
                        </div>
                    }))
                }
            }
        }}
    }
}
