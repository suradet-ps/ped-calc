//! Dose formatting utilities: mg → mL conversion, rounding, and display strings.
#![allow(dead_code)]

use crate::types::{
    calculation::DisplayDose,
    drug::{Drug, Formulation},
};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;

/// Convert a dose in mg to mL using the formulation's concentration.
pub fn mg_to_ml(dose_mg: Decimal, formulation: &Formulation) -> String {
    if formulation.mg_per_ml == Decimal::ZERO {
        return "N/A".to_string();
    }
    let ml = dose_mg / formulation.mg_per_ml;
    format_volume_ml(ml)
}

/// Format a volume in mL as a human-readable string.
pub fn format_volume_ml(ml: Decimal) -> String {
    if ml.fract() == Decimal::ZERO {
        format!("{} mL", ml.to_i64().unwrap_or(0))
    } else {
        let rounded = ml.round_dp(1);
        format!("{} mL", rounded)
    }
}

/// Format a dose in mg as a human-readable string.
pub fn format_dose_mg(dose_mg: Decimal) -> String {
    if dose_mg.fract() == Decimal::ZERO {
        format!("{} mg", dose_mg.to_i64().unwrap_or(0))
    } else {
        format!("{} mg", dose_mg.round_dp(1))
    }
}

/// Round a dose to a practical tablet/liquid amount.
///
/// Rules:
/// - dose < 10 mg → round to 1 decimal place
/// - dose 10–99 mg → round to nearest 0.5 mg
/// - dose ≥ 100 mg → round to nearest 5 mg
pub fn round_dose_to_practical(dose_mg: Decimal) -> Decimal {
    if dose_mg < Decimal::from(10) {
        dose_mg.round_dp(1)
    } else if dose_mg < Decimal::from(100) {
        (dose_mg * Decimal::from(2)).round_dp(0) / Decimal::from(2)
    } else {
        (dose_mg / Decimal::from(5)).round_dp(0) * Decimal::from(5)
    }
}

/// Build a complete `DisplayDose` from a calculated dose and drug data.
/// Automatically selects the default formulation if none is specified.
pub fn build_full_display_dose(
    dose_mg: Decimal,
    raw_dose_mg: Decimal,
    drug: &Drug,
    selected_formulation: Option<&Formulation>,
) -> DisplayDose {
    let mg_string = format_dose_mg(dose_mg);

    let formulation =
        selected_formulation.or_else(|| drug.formulations.iter().find(|f| f.is_default));

    let ml_string = formulation.map(|f| {
        if f.mg_per_ml == Decimal::ZERO {
            return "N/A".to_string();
        }
        let ml = dose_mg / f.mg_per_ml;
        format_volume_ml(ml)
    });

    let formulation_used = formulation.map(|f| f.description.clone());

    let rounding_note = if (dose_mg - raw_dose_mg).abs() > Decimal::new(1, 1) {
        Some(format!("ปัดจาก {} mg", raw_dose_mg.round_dp(1)))
    } else {
        None
    };

    DisplayDose {
        mg_string,
        ml_string,
        formulation_used,
        rounding_note,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_round_dose_small() {
        assert_eq!(round_dose_to_practical(dec!(5.37)), dec!(5.4));
    }

    #[test]
    fn test_round_dose_medium() {
        assert_eq!(round_dose_to_practical(dec!(67.3)), dec!(67.5));
    }

    #[test]
    fn test_round_dose_large() {
        assert_eq!(round_dose_to_practical(dec!(267)), dec!(265));
    }

    #[test]
    fn test_mg_to_ml_basic() {
        let formulation = Formulation {
            description: "Syrup 125mg/5mL".into(),
            mg_per_ml: dec!(25),
            is_default: true,
        };
        assert_eq!(mg_to_ml(dec!(250), &formulation), "10 mL");
    }

    #[test]
    fn test_mg_to_ml_fractional() {
        let formulation = Formulation {
            description: "Syrup 125mg/5mL".into(),
            mg_per_ml: dec!(25),
            is_default: true,
        };
        assert_eq!(mg_to_ml(dec!(125), &formulation), "5 mL");
    }

    #[test]
    fn test_format_dose_mg_whole() {
        assert_eq!(format_dose_mg(dec!(500)), "500 mg");
    }

    #[test]
    fn test_format_dose_mg_fractional() {
        assert_eq!(format_dose_mg(dec!(37.5)), "37.5 mg");
    }
}
