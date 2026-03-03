use crate::components::{
    dose_result::DoseResult, drug_selector::DrugSelector, history_panel::HistoryPanel,
    patient_form::PatientForm,
};
use leptos::prelude::*;

#[component]
pub fn CalculatorPage() -> impl IntoView {
    view! {
        <div class="calculator-page">
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                // Left Column: Input
                <div class="lg:col-span-2 space-y-6">
                    // Drug Selector Card
                    <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-4">
                        <DrugSelector />
                    </div>

                    // Patient Form Card
                    <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-4">
                        <PatientForm />
                    </div>

                    // Dose Result Card
                    <DoseResult />
                </div>

                // Right Column: History
                <div class="space-y-6">
                    <HistoryPanel />
                </div>
            </div>
        </div>
    }
}
