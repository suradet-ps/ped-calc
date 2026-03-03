use crate::data::drugs::all_drugs;
use leptos::prelude::*;

#[component]
pub fn DrugReferencePage() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let all = all_drugs();
    let drugs_data = StoredValue::new(all);

    let filtered = Memo::new(move |_| {
        let q = search.get();
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

    view! {
        <div class="drug-reference-page">
            <h1 class="text-2xl font-bold text-gray-900 mb-6">"ตำราอ้างอิงยา"</h1>

            <input
                type="search"
                placeholder="ค้นหายา..."
                class="w-full border border-gray-300 rounded-lg px-4 py-2 mb-6 focus:outline-none focus:ring-2 focus:ring-blue-300"
                on:input=move |ev| search.set(event_target_value(&ev))
                aria-label="ค้นหายา"
            />

            <div class="space-y-4">
                {move || {
                    let drugs = filtered.get();
                    drugs.into_iter().map(|drug| {
                        let dosings = drug.age_dosings.clone();
                        view! {
                            <div class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
                                // Drug header
                                <div class="p-4 bg-gradient-to-r from-blue-50 to-white border-b border-gray-100">
                                    <div class="flex items-center justify-between">
                                        <div>
                                            <h2 class="font-semibold text-gray-900">{drug.generic_name.clone()}</h2>
                                            <p class="text-xs text-gray-500">{drug.brand_names.join(", ")}</p>
                                        </div>
                                        <span class="text-xs rounded-full px-2 py-0.5 bg-blue-100 text-blue-700">
                                            {drug.category.display_en().to_string()}
                                        </span>
                                    </div>
                                </div>

                                // Dosing table
                                <div class="overflow-x-auto">
                                    <table class="w-full text-sm">
                                        <thead>
                                            <tr class="text-left text-xs text-gray-500 border-b border-gray-100">
                                                <th class="px-4 py-2">"กลุ่มอายุ"</th>
                                                <th class="px-4 py-2">"Dose (mg/kg)"</th>
                                                <th class="px-4 py-2">"Max/dose"</th>
                                                <th class="px-4 py-2">"Max/day"</th>
                                                <th class="px-4 py-2">"Freq"</th>
                                                <th class="px-4 py-2">"Route"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {dosings.into_iter().map(|ad| {
                                                let dose_str = if ad.dose_range.min_mg_per_kg == ad.dose_range.max_mg_per_kg {
                                                    if ad.dose_range.min_mg_per_kg > rust_decimal::Decimal::ZERO {
                                                        format!("{}", ad.dose_range.min_mg_per_kg)
                                                    } else {
                                                        "Flat dose".to_string()
                                                    }
                                                } else {
                                                    format!("{}-{}", ad.dose_range.min_mg_per_kg, ad.dose_range.max_mg_per_kg)
                                                };
                                                view! {
                                                    <tr class="border-b border-gray-50 hover:bg-gray-50">
                                                        <td class="px-4 py-2 font-medium">{ad.age_group.display_th().to_string()}</td>
                                                        <td class="px-4 py-2">{dose_str}</td>
                                                        <td class="px-4 py-2">{
                                                            ad.dose_range.max_single_dose_mg
                                                                .map(|m| format!("{} mg", m))
                                                                .unwrap_or_else(|| "-".to_string())
                                                        }</td>
                                                        <td class="px-4 py-2">{
                                                            ad.dose_range.max_daily_dose_mg
                                                                .map(|m| format!("{} mg", m))
                                                                .unwrap_or_else(|| "-".to_string())
                                                        }</td>
                                                        <td class="px-4 py-2">{ad.default_frequency.abbreviation()}</td>
                                                        <td class="px-4 py-2">{ad.route.abbreviation().to_string()}</td>
                                                    </tr>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </tbody>
                                    </table>
                                </div>

                                // Contraindications, reference
                                <div class="px-4 py-3 bg-gray-50 text-xs text-gray-500 space-y-1">
                                    {(!drug.contraindications.is_empty()).then(|| view! {
                                        <p><strong>"ข้อห้ามใช้: "</strong>{drug.contraindications.join("; ")}</p>
                                    })}
                                    <p>{format!("อ้างอิง: {} | Last reviewed: {}", drug.reference, drug.last_reviewed)}</p>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
