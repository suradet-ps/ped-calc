//! Pure dose calculation functions (no framework dependencies).
//! Flow follows DESIGN.md section 4.

use rust_decimal::Decimal;

use crate::logic::formatter;
use crate::types::{
    calculation::{DoseError, DoseResult, DoseWarning},
    drug::{AgeDosing, Drug, WeightBasis},
    patient::{AgeGroup, PatientData},
};

/// Weight threshold (kg) for adult dose capping.
const ADULT_WEIGHT_THRESHOLD: Decimal = Decimal::from_parts(50, 0, 0, false, 0);

/// Percentage threshold for near-max dose warnings.
const NEAR_MAX_PERCENT: Decimal = Decimal::from_parts(80, 0, 0, false, 0);

/// Calculate the recommended dose for a given drug and patient.
///
/// # Steps
/// 1. Validate inputs
/// 2. Determine age group
/// 3. Find age-specific dosing protocol
/// 4. Select weight (actual vs IBW vs adjusted)
/// 5. Compute raw dose (mg/kg × weight or flat dose)
/// 6. Apply caps (single dose, daily dose, adult flat dose)
/// 7. Collect clinical warnings (renal, hepatic, neonatal, pregnancy)
/// 8. Sort warnings by severity (critical first)
/// 9. Build formatted display dose
pub fn calculate_dose(drug: &Drug, patient: &PatientData) -> Result<DoseResult, DoseError> {
    // Validate patient data
    patient
        .is_valid_for_calculation()
        .map_err(DoseError::IncompletePatientData)?;

    // Determine age group
    let age_group = patient.age_group();

    // Find appropriate dosing protocol
    let age_dosing = drug
        .find_dosing_for_age(&age_group)
        .ok_or_else(|| DoseError::NoDoseForAgeGroup(age_group.display_th().to_string()))?;

    // Select dosing weight
    let (weight_used, weight_warnings) = select_weight(patient, age_dosing)?;

    // Compute raw dose
    let (dose_per_kg, raw_dose_mg) = compute_raw_dose(weight_used, age_dosing);

    // Apply dose caps
    let (capped_dose_mg, cap_warnings) = apply_dose_caps(raw_dose_mg, weight_used, age_dosing);

    // Collect clinical warnings
    let mut all_warnings: Vec<DoseWarning> = Vec::new();
    all_warnings.extend(weight_warnings);
    all_warnings.extend(cap_warnings);

    // Renal / hepatic adjustment warnings
    if drug.requires_renal_adjustment && patient.renal_function.is_impaired() {
        all_warnings.push(DoseWarning::RenalAdjustmentNeeded);
    }
    if drug.requires_hepatic_adjustment && patient.hepatic_function.is_impaired() {
        all_warnings.push(DoseWarning::HepaticAdjustmentNeeded);
    }

    // Neonatal caution warning
    if matches!(age_group, AgeGroup::Neonate) {
        if let Some(ref notes) = age_dosing.clinical_notes {
            all_warnings.push(DoseWarning::NeonatalCaution {
                message: notes.clone(),
            });
        }
    }

    // Pregnancy risk warning
    if patient.is_pregnant {
        if let Some(ref preg_cat) = drug.pregnancy_category {
            if matches!(preg_cat.as_str(), "C" | "D" | "X") {
                all_warnings.push(DoseWarning::PregnancyRisk {
                    category: preg_cat.clone(),
                    message: pregnancy_message(preg_cat),
                });
            }
        }
    }

    // Breastfeeding caution
    if patient.is_breastfeeding && drug.contraindicated_in_breastfeeding {
        all_warnings.push(DoseWarning::BreastfeedingCaution {
            message: "ยานี้อาจเป็นอันตรายต่อทารกผ่านน้ำนม — กรุณาปรึกษาแพทย์".into(),
        });
    }

    // Off-label use warning
    if age_dosing.is_off_label {
        all_warnings.push(DoseWarning::OffLabelUse {
            age_group: age_group.clone(),
            message: format!(
                "การใช้ยานี้ใน{} ยังไม่ได้รับการอนุมัติอย่างเป็นทางการ",
                age_group.display_th()
            ),
        });
    }

    // Check daily dose cap
    let daily_dose_mg = match age_dosing.default_frequency.doses_per_day() {
        Some(doses) => capped_dose_mg * doses,
        None => capped_dose_mg,
    };

    if let Some(max_daily) = age_dosing.dose_range.max_daily_dose_mg {
        if daily_dose_mg > max_daily {
            all_warnings.push(DoseWarning::ExceedsMaxDailyDose {
                calculated_mg: daily_dose_mg,
                max_allowed_mg: max_daily,
            });
        } else {
            let pct = (daily_dose_mg / max_daily) * Decimal::from(100);
            if pct >= NEAR_MAX_PERCENT {
                all_warnings.push(DoseWarning::NearMaxDose {
                    percentage_of_max: pct,
                    is_single_dose: false,
                });
            }
        }
    }

    // Sort warnings by severity (critical first)
    all_warnings.sort_by_key(|w| std::cmp::Reverse(w.severity()));

    // Build dose range
    let min_dose_mg = if age_dosing.dose_range.min_mg_per_kg > Decimal::ZERO {
        age_dosing.dose_range.min_mg_per_kg * weight_used
    } else if let Some(flat) = age_dosing.dose_range.adult_flat_dose_mg {
        flat
    } else {
        capped_dose_mg
    };

    let max_dose_mg = match age_dosing.dose_range.max_single_dose_mg {
        Some(max) => {
            if age_dosing.dose_range.max_mg_per_kg > Decimal::ZERO {
                max.min(age_dosing.dose_range.max_mg_per_kg * weight_used)
            } else {
                max
            }
        }
        None => {
            if age_dosing.dose_range.max_mg_per_kg > Decimal::ZERO {
                age_dosing.dose_range.max_mg_per_kg * weight_used
            } else {
                capped_dose_mg
            }
        }
    };

    // Build formatted display dose
    let display_dose = formatter::build_full_display_dose(capped_dose_mg, raw_dose_mg, drug, None);

    Ok(DoseResult {
        single_dose_mg: capped_dose_mg,
        daily_dose_mg,
        frequency: age_dosing.default_frequency.clone(),
        min_single_dose_mg: min_dose_mg,
        max_single_dose_mg: max_dose_mg,
        weight_used_kg: weight_used,
        age_group_used: age_group,
        dose_per_kg,
        warnings: all_warnings,
        display_dose,
    })
}

/// Compute raw dose: either weight-based or flat adult dose
fn compute_raw_dose(weight_kg: Decimal, age_dosing: &AgeDosing) -> (Decimal, Decimal) {
    // If adult flat dose is available and weight >= 50, use flat dose
    if weight_kg >= ADULT_WEIGHT_THRESHOLD {
        if let Some(flat) = age_dosing.dose_range.adult_flat_dose_mg {
            return (Decimal::ZERO, flat);
        }
    }

    // Weight-based dosing
    let dose_per_kg = age_dosing.dose_range.max_mg_per_kg;
    if dose_per_kg > Decimal::ZERO {
        let raw = dose_per_kg * weight_kg;
        (dose_per_kg, raw)
    } else if let Some(flat) = age_dosing.dose_range.adult_flat_dose_mg {
        // Flat dose entries with 0 mg/kg
        (Decimal::ZERO, flat)
    } else {
        (Decimal::ZERO, Decimal::ZERO)
    }
}

/// Select the appropriate body weight for dosing based on the weight basis.
fn select_weight(
    patient: &PatientData,
    age_dosing: &AgeDosing,
) -> Result<(Decimal, Vec<DoseWarning>), DoseError> {
    let mut warnings = Vec::new();

    let weight = match age_dosing.dose_range.weight_basis {
        WeightBasis::Actual => patient.weight_kg,

        WeightBasis::Ideal => match patient.ideal_body_weight_kg() {
            Some(ibw) if ibw < patient.weight_kg => {
                warnings.push(DoseWarning::IdealBodyWeightUsed {
                    actual_weight_kg: patient.weight_kg,
                    ibw_used_kg: ibw,
                });
                ibw
            }
            _ => patient.weight_kg,
        },

        WeightBasis::Adjusted => match patient.ideal_body_weight_kg() {
            Some(ibw) if patient.weight_kg > ibw => {
                let adjusted = ibw + Decimal::new(4, 1) * (patient.weight_kg - ibw);
                warnings.push(DoseWarning::IdealBodyWeightUsed {
                    actual_weight_kg: patient.weight_kg,
                    ibw_used_kg: adjusted,
                });
                adjusted
            }
            _ => patient.weight_kg,
        },

        WeightBasis::Lean => patient.weight_kg,
    };

    Ok((weight, warnings))
}

/// Apply dose caps: single dose, adult flat dose
fn apply_dose_caps(
    raw_dose_mg: Decimal,
    weight_kg: Decimal,
    age_dosing: &AgeDosing,
) -> (Decimal, Vec<DoseWarning>) {
    let mut warnings = Vec::new();
    let mut dose = raw_dose_mg;

    // Adult dose cap: use flat dose when weight ≥ 50 kg
    if weight_kg >= ADULT_WEIGHT_THRESHOLD {
        if let Some(adult_dose) = age_dosing.dose_range.adult_flat_dose_mg {
            if raw_dose_mg != adult_dose && age_dosing.dose_range.max_mg_per_kg > Decimal::ZERO {
                warnings.push(DoseWarning::AdultDoseCapped {
                    calculated_by_weight_mg: raw_dose_mg,
                    adult_dose_used_mg: adult_dose,
                });
                dose = adult_dose;
            }
        }
    }

    // Max single dose cap
    if let Some(max_single) = age_dosing.dose_range.max_single_dose_mg {
        if dose > max_single {
            warnings.push(DoseWarning::ExceedsMaxSingleDose {
                calculated_mg: dose,
                max_allowed_mg: max_single,
                capped_at_mg: max_single,
            });
            dose = max_single;
        } else if max_single > Decimal::ZERO {
            let percentage = (dose / max_single) * Decimal::from(100);
            if percentage >= NEAR_MAX_PERCENT {
                warnings.push(DoseWarning::NearMaxDose {
                    percentage_of_max: percentage,
                    is_single_dose: true,
                });
            }
        }
    }

    (dose, warnings)
}

/// Returns a localized pregnancy warning message for the given FDA category.
fn pregnancy_message(category: &str) -> String {
    match category {
        "C" => "ไม่มีข้อมูลความปลอดภัยที่เพียงพอในมนุษย์ — ใช้เมื่อประโยชน์มากกว่าความเสี่ยง".into(),
        "D" => "มีหลักฐานความเสี่ยงต่อทารกในครรภ์ — ใช้เฉพาะกรณีจำเป็นเร่งด่วน".into(),
        "X" => "ห้ามใช้ระหว่างตั้งครรภ์ — มีหลักฐานชัดเจนว่าเป็นอันตรายต่อทารก".into(),
        _ => "กรุณาปรึกษาแพทย์".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::drugs::get_drug_by_id;
    use crate::types::patient::{HepaticFunction, RenalFunction, Sex};
    use rust_decimal_macros::dec;

    fn make_patient(weight: Decimal, age_years: u8) -> PatientData {
        PatientData {
            weight_kg: weight,
            age_years: Some(age_years),
            age_months: None,
            age_days: None,
            sex: Some(Sex::Male),
            renal_function: RenalFunction::Normal,
            hepatic_function: HepaticFunction::Normal,
            is_pregnant: false,
            is_breastfeeding: false,
        }
    }

    #[test]
    fn test_normal_dose_calculation() {
        let drug = get_drug_by_id("amoxicillin-oral").expect("drug not found");
        let patient = make_patient(dec!(15), 5);
        let result = calculate_dose(&drug, &patient);
        assert!(
            result.is_ok(),
            "should calculate successfully: {:?}",
            result
        );
        let dose = result.unwrap();
        // 15 kg × 40 mg/kg = 600 mg but max single = 500 mg → capped
        assert_eq!(dose.single_dose_mg, dec!(500));
    }

    #[test]
    fn test_adult_dose_cap_at_50kg() {
        let drug = get_drug_by_id("amoxicillin-oral").expect("drug not found");
        let patient = make_patient(dec!(55), 16); // Adolescent
        let result = calculate_dose(&drug, &patient).expect("calc failed");
        // Should use adult flat dose = 500 mg (compute_raw_dose returns flat directly)
        assert_eq!(result.single_dose_mg, dec!(500));
    }

    #[test]
    fn test_renal_impairment_warning() {
        let drug = get_drug_by_id("amoxicillin-oral").expect("drug not found");
        let mut patient = make_patient(dec!(20), 8);
        patient.renal_function = RenalFunction::SevereImpairment;
        let result = calculate_dose(&drug, &patient).expect("calc failed");
        assert!(
            result
                .warnings
                .iter()
                .any(|w| matches!(w, DoseWarning::RenalAdjustmentNeeded)),
            "should have renal warning"
        );
    }

    #[test]
    fn test_invalid_weight_returns_error() {
        let drug = get_drug_by_id("amoxicillin-oral").expect("drug not found");
        let patient = make_patient(dec!(0), 5);
        let result = calculate_dose(&drug, &patient);
        assert!(matches!(result, Err(DoseError::IncompletePatientData(_))));
    }

    #[test]
    fn test_neonate_dosing_selection() {
        let drug = get_drug_by_id("amoxicillin-oral").expect("drug not found");
        let patient = PatientData {
            weight_kg: dec!(3.5),
            age_days: Some(7),
            ..Default::default()
        };
        let result = calculate_dose(&drug, &patient).expect("calc failed");
        assert!(matches!(result.age_group_used, AgeGroup::Neonate));
    }

    #[test]
    fn test_paracetamol_child_dose() {
        let drug = get_drug_by_id("paracetamol-oral").expect("drug not found");
        let patient = make_patient(dec!(20), 5); // 20 kg, 5 yrs
        let result = calculate_dose(&drug, &patient).expect("calc failed");
        // 20 × 15 = 300 mg per dose
        assert_eq!(result.single_dose_mg, dec!(300));
    }

    #[test]
    fn test_cetirizine_flat_dose_adolescent() {
        let drug = get_drug_by_id("cetirizine-oral").expect("drug not found");
        let patient = make_patient(dec!(50), 14); // 50 kg, 14 yrs → Adolescent
        let result = calculate_dose(&drug, &patient).expect("calc failed");
        assert_eq!(result.single_dose_mg, dec!(10));
    }

    #[test]
    fn test_no_dose_for_neonate_ibuprofen() {
        let drug = get_drug_by_id("ibuprofen-oral").expect("drug not found");
        let patient = PatientData {
            weight_kg: dec!(3.5),
            age_days: Some(7),
            ..Default::default()
        };
        let result = calculate_dose(&drug, &patient);
        // Ibuprofen has no Neonate age group
        assert!(matches!(result, Err(DoseError::NoDoseForAgeGroup(_))));
    }
}
