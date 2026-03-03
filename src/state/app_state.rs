//! Global application state — distributed via Leptos Context.
//! Leptos 0.7 CSR — uses RwSignal, Memo, Effect.

use crate::logic::calculator::calculate_dose;
use crate::types::{
    calculation::{DoseError, DoseResult, DoseWarning},
    drug::{Drug, Frequency},
    patient::PatientData,
};
use leptos::prelude::*;

/// A single calculation history entry.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: String,
    pub drug_id: String,
    pub drug_display_name: String,
    pub patient_weight_kg: String,
    pub single_dose_mg: String,
    pub frequency: String,
    pub age_group: String,
    pub had_critical_warning: bool,
}

/// Application settings.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    pub show_ml_equivalent: bool,
    pub dark_mode: bool,
    pub history_max_entries: usize,
    pub default_concentration_unit: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            show_ml_equivalent: true,
            dark_mode: false,
            history_max_entries: 50,
            default_concentration_unit: "mg_per_ml".to_string(),
        }
    }
}

/// Global App State
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct AppState {
    // Input signals
    pub selected_drug: RwSignal<Option<Drug>>,
    pub patient_data: RwSignal<PatientData>,
    pub override_frequency: RwSignal<Option<Frequency>>,
    pub search_query: RwSignal<String>,

    // UI state
    pub is_calculating: RwSignal<bool>,
    pub show_confirm_modal: RwSignal<bool>,
    pub user_confirmed_warning: RwSignal<bool>,
    pub active_tab: RwSignal<String>,

    // Derived state
    pub dose_result: Memo<Option<Result<DoseResult, DoseError>>>,
    pub active_warnings: Memo<Vec<DoseWarning>>,
    pub has_critical_warning: Memo<bool>,
    pub can_submit: Memo<bool>,

    // Persistent state
    pub history: RwSignal<Vec<HistoryEntry>>,
    pub settings: RwSignal<AppSettings>,
}

impl AppState {
    /// Create a new `AppState` — called once in the root `App` component.
    pub fn new() -> Self {
        let selected_drug = RwSignal::new(None::<Drug>);
        let patient_data = RwSignal::new(PatientData::default());
        let override_frequency = RwSignal::new(None::<Frequency>);
        let search_query = RwSignal::new(String::new());

        let is_calculating = RwSignal::new(false);
        let show_confirm_modal = RwSignal::new(false);
        let user_confirmed_warning = RwSignal::new(false);
        let active_tab = RwSignal::new("calculator".to_string());

        let history = RwSignal::new(load_history_from_storage());
        let settings = RwSignal::new(load_settings_from_storage());

        // Derived: dose_result (auto-recalculates on input change)
        let dose_result = Memo::new(move |_| {
            let drug = selected_drug.get();
            let patient = patient_data.get();

            match drug {
                None => None,
                Some(d) => {
                    if patient.weight_kg <= rust_decimal::Decimal::ZERO {
                        None
                    } else {
                        Some(calculate_dose(&d, &patient))
                    }
                }
            }
        });

        // Derived: active_warnings
        let active_warnings = Memo::new(move |_| match dose_result.get() {
            Some(Ok(ref result)) => result.warnings.clone(),
            _ => Vec::new(),
        });

        // Derived: has_critical_warning
        let has_critical_warning = Memo::new(move |_| match dose_result.get() {
            Some(Ok(ref result)) => result.has_critical_warning(),
            _ => false,
        });

        // Derived: can_submit
        let can_submit = Memo::new(move |_| {
            let drug_selected = selected_drug.get().is_some();
            let patient_valid = patient_data.get().weight_kg > rust_decimal::Decimal::ZERO;
            let critical_confirmed = !has_critical_warning.get() || user_confirmed_warning.get();
            drug_selected && patient_valid && critical_confirmed
        });

        // Effect: reset confirmation when inputs change
        Effect::new(move |_| {
            let _ = selected_drug.get();
            let _ = patient_data.get();
            user_confirmed_warning.set(false);
            show_confirm_modal.set(false);
        });

        // Effect: persist history to localStorage
        Effect::new(move |_| {
            let h = history.get();
            save_history_to_storage(&h);
        });

        // Effect: persist settings to localStorage
        Effect::new(move |_| {
            let s = settings.get();
            save_settings_to_storage(&s);
        });

        Self {
            selected_drug,
            patient_data,
            override_frequency,
            search_query,
            is_calculating,
            show_confirm_modal,
            user_confirmed_warning,
            active_tab,
            dose_result,
            active_warnings,
            has_critical_warning,
            can_submit,
            history,
            settings,
        }
    }

    /// Add a history entry (most recent first, capped at 50).
    pub fn add_to_history(&self, entry: HistoryEntry) {
        self.history.update(|h| {
            h.insert(0, entry);
            h.truncate(50);
        });
    }

    /// Remove a history entry by ID.
    pub fn remove_history_entry(&self, id: &str) {
        self.history.update(|h| {
            h.retain(|e| e.id != id);
        });
    }

    /// Clear all history entries.
    pub fn clear_history(&self) {
        self.history.set(Vec::new());
    }

    /// Reset the patient form to defaults.
    #[allow(dead_code)]
    pub fn reset_patient(&self) {
        self.patient_data.set(PatientData::default());
    }

    /// Reset all state to defaults.
    #[allow(dead_code)]
    pub fn reset_all(&self) {
        self.selected_drug.set(None);
        self.patient_data.set(PatientData::default());
        self.override_frequency.set(None);
        self.search_query.set(String::new());
        self.user_confirmed_warning.set(false);
    }
}

// ===== localStorage helpers =====

const HISTORY_KEY: &str = "pedcalc_history";
const SETTINGS_KEY: &str = "pedcalc_settings";

fn get_local_storage() -> Option<web_sys::Storage> {
    let window = web_sys::window()?;
    window.local_storage().ok()?
}

fn load_history_from_storage() -> Vec<HistoryEntry> {
    let storage = match get_local_storage() {
        Some(s) => s,
        None => return Vec::new(),
    };
    let json = match storage.get_item(HISTORY_KEY) {
        Ok(Some(j)) => j,
        _ => return Vec::new(),
    };
    serde_json::from_str(&json).unwrap_or_default()
}

fn save_history_to_storage(history: &[HistoryEntry]) {
    let storage = match get_local_storage() {
        Some(s) => s,
        None => return,
    };
    if let Ok(json) = serde_json::to_string(history) {
        let _ = storage.set_item(HISTORY_KEY, &json);
    }
}

fn load_settings_from_storage() -> AppSettings {
    let storage = match get_local_storage() {
        Some(s) => s,
        None => return AppSettings::default(),
    };
    let json = match storage.get_item(SETTINGS_KEY) {
        Ok(Some(j)) => j,
        _ => return AppSettings::default(),
    };
    serde_json::from_str(&json).unwrap_or_default()
}

fn save_settings_to_storage(settings: &AppSettings) {
    let storage = match get_local_storage() {
        Some(s) => s,
        None => return,
    };
    if let Ok(json) = serde_json::to_string(settings) {
        let _ = storage.set_item(SETTINGS_KEY, &json);
    }
}
