use crate::logic::validator;
use crate::state::app_state::AppState;
use crate::types::patient::{HepaticFunction, RenalFunction, Sex};
use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
enum AgeInputMode {
    Years,
    Months,
    Days,
}

#[component]
pub fn PatientForm() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");

    let age_mode = RwSignal::new(AgeInputMode::Years);
    let weight_error = RwSignal::new(None::<String>);
    let age_value = RwSignal::new(String::new());

    // Weight input handler
    let on_weight_change = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        match Decimal::from_str(&value) {
            Ok(w) => match validator::validate_weight(w) {
                Ok(()) => {
                    weight_error.set(None);
                    state.patient_data.update(|p| p.weight_kg = w);
                }
                Err(e) => {
                    weight_error.set(Some(e));
                }
            },
            Err(_) => {
                if value.is_empty() {
                    weight_error.set(None);
                    state.patient_data.update(|p| p.weight_kg = Decimal::ZERO);
                } else {
                    weight_error.set(Some("กรุณากรอกตัวเลขที่ถูกต้อง".to_string()));
                }
            }
        }
    };

    // Age input handler
    let on_age_change = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        age_value.set(value.clone());

        if value.is_empty() {
            state.patient_data.update(|p| {
                p.age_years = None;
                p.age_months = None;
                p.age_days = None;
            });
            return;
        }

        if let Ok(v) = value.parse::<u16>() {
            let mode = age_mode.get_untracked();
            state.patient_data.update(|p| {
                p.age_years = None;
                p.age_months = None;
                p.age_days = None;
                match mode {
                    AgeInputMode::Years => {
                        p.age_years = Some(v as u8);
                    }
                    AgeInputMode::Months => {
                        p.age_months = Some(v as u8);
                    }
                    AgeInputMode::Days => {
                        p.age_days = Some(v);
                    }
                }
            });
        }
    };

    // Renal function handler
    let on_renal_change = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        let renal = match value.as_str() {
            "mild" => RenalFunction::MildImpairment,
            "moderate" => RenalFunction::ModerateImpairment,
            "severe" => RenalFunction::SevereImpairment,
            "esrd" => RenalFunction::EndStageRenalDisease,
            _ => RenalFunction::Normal,
        };
        state.patient_data.update(|p| p.renal_function = renal);
    };

    // Hepatic function handler
    let on_hepatic_change = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        let hepatic = match value.as_str() {
            "mild" => HepaticFunction::MildImpairment,
            "moderate" => HepaticFunction::ModerateImpairment,
            "severe" => HepaticFunction::SevereImpairment,
            _ => HepaticFunction::Normal,
        };
        state.patient_data.update(|p| p.hepatic_function = hepatic);
    };

    // Sex handler
    let on_sex_change = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        let sex = match value.as_str() {
            "male" => Some(Sex::Male),
            "female" => Some(Sex::Female),
            _ => None,
        };
        state.patient_data.update(|p| p.sex = sex);
    };

    view! {
        <form class="patient-form space-y-4" on:submit=|ev: web_sys::SubmitEvent| ev.prevent_default()>
            <h2 class="text-base font-semibold text-gray-800">"ข้อมูลผู้ป่วย"</h2>

            // Weight Field
            <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">
                    "น้ำหนัก (kg) "
                    <span class="text-red-500">"*"</span>
                </label>
                <div class="relative">
                    <input
                        type="number"
                        class=move || {
                            let base = "w-full border rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2";
                            if weight_error.get().is_some() {
                                format!("{base} border-red-400 focus:ring-red-300")
                            } else {
                                format!("{base} border-gray-300 focus:ring-blue-300")
                            }
                        }
                        placeholder="เช่น 15.5"
                        min="0.1"
                        max="200"
                        step="0.1"
                        aria-label="น้ำหนักผู้ป่วย (กิโลกรัม)"
                        on:input=on_weight_change
                    />
                    <span class="absolute right-3 top-2.5 text-gray-400 text-sm">"kg"</span>
                </div>
                {move || weight_error.get().map(|e| view! {
                    <p class="text-red-600 text-xs mt-1" role="alert">{e}</p>
                })}
            </div>

            // Age Mode Selector
            <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">"อายุ"</label>
                <div class="flex gap-2 mb-2">
                    {["Years", "Months", "Days"].into_iter().map(|mode_str| {
                        let mode_val = match mode_str {
                            "Years" => AgeInputMode::Years,
                            "Months" => AgeInputMode::Months,
                            _ => AgeInputMode::Days,
                        };
                        let label = match mode_str {
                            "Years" => "ปี",
                            "Months" => "เดือน",
                            _ => "วัน",
                        };
                        let mode_clone = mode_val.clone();
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    if age_mode.get() == mode_clone {
                                        "px-3 py-1 text-sm rounded-full bg-blue-600 text-white"
                                    } else {
                                        "px-3 py-1 text-sm rounded-full bg-gray-100 text-gray-600 hover:bg-gray-200"
                                    }
                                }
                                on:click={
                                    let mv = mode_val.clone();
                                    move |_| {
                                        age_mode.set(mv.clone());
                                        age_value.set(String::new());
                                        state.patient_data.update(|p| {
                                            p.age_years = None;
                                            p.age_months = None;
                                            p.age_days = None;
                                        });
                                    }
                                }
                            >
                                {label}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                <input
                    type="number"
                    class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-300"
                    placeholder=move || match age_mode.get() {
                        AgeInputMode::Years => "อายุ (ปี)",
                        AgeInputMode::Months => "อายุ (เดือน)",
                        AgeInputMode::Days => "อายุ (วัน)",
                    }
                    min="0"
                    max=move || match age_mode.get() {
                        AgeInputMode::Years => "120",
                        AgeInputMode::Months => "23",
                        AgeInputMode::Days => "28",
                    }
                    aria-label="อายุผู้ป่วย"
                    prop:value=move || age_value.get()
                    on:input=on_age_change
                />

                // Age group badge
                {move || {
                    let patient = state.patient_data.get();
                    let has_age = patient.age_years.is_some() || patient.age_months.is_some() || patient.age_days.is_some();
                    if has_age {
                        let age_group = patient.age_group();
                        view! {
                            <span class="inline-block mt-1.5 text-xs rounded-full px-2.5 py-0.5 bg-purple-100 text-purple-700 font-medium">
                                {format!("กลุ่มอายุ: {}", age_group.display_th())}
                            </span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}
            </div>

            // Sex
            <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">"เพศ"</label>
                <select
                    class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-300"
                    aria-label="เพศ"
                    on:change=on_sex_change
                >
                    <option value="">"ไม่ระบุ"</option>
                    <option value="male">"ชาย"</option>
                    <option value="female">"หญิง"</option>
                </select>
            </div>

            // Renal Function
            <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">"การทำงานของไต"</label>
                <select
                    class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-300"
                    aria-label="ภาวะการทำงานของไต"
                    on:change=on_renal_change
                >
                    <option value="normal">"ปกติ"</option>
                    <option value="mild">"บกพร่องเล็กน้อย (eGFR 60–89)"</option>
                    <option value="moderate">"บกพร่องปานกลาง (eGFR 30–59)"</option>
                    <option value="severe">"บกพร่องรุนแรง (eGFR 15–29)"</option>
                    <option value="esrd">"ไตวายเรื้อรังระยะสุดท้าย / ฟอกไต"</option>
                </select>
            </div>

            // Hepatic Function
            <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">"การทำงานของตับ"</label>
                <select
                    class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-300"
                    aria-label="ภาวะการทำงานของตับ"
                    on:change=on_hepatic_change
                >
                    <option value="normal">"ปกติ"</option>
                    <option value="mild">"บกพร่องเล็กน้อย (Child-Pugh A)"</option>
                    <option value="moderate">"บกพร่องปานกลาง (Child-Pugh B)"</option>
                    <option value="severe">"บกพร่องรุนแรง (Child-Pugh C)"</option>
                </select>
            </div>

            // Special conditions
            <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">"ภาวะพิเศษ"</label>
                <div class="space-y-2">
                    <label class="flex items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            class="accent-blue-600"
                            aria-label="ตั้งครรภ์"
                            on:change=move |ev: web_sys::Event| {
                                let checked = event_target_checked(&ev);
                                state.patient_data.update(|p| p.is_pregnant = checked);
                            }
                        />
                        <span class="text-sm text-gray-700">"ตั้งครรภ์"</span>
                    </label>
                    <label class="flex items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            class="accent-blue-600"
                            aria-label="ให้นมบุตร"
                            on:change=move |ev: web_sys::Event| {
                                let checked = event_target_checked(&ev);
                                state.patient_data.update(|p| p.is_breastfeeding = checked);
                            }
                        />
                        <span class="text-sm text-gray-700">"ให้นมบุตร"</span>
                    </label>
                </div>
            </div>
        </form>
    }
}

fn event_target_checked(ev: &web_sys::Event) -> bool {
    use wasm_bindgen::JsCast;
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|el| el.checked())
        .unwrap_or(false)
}
