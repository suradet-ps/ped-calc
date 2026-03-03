//! Renal and hepatic dose adjustments.
//!
//! Drug-specific adjustments per DRUG_DATABASE.md "Renal Adjustment Table".
#![allow(dead_code)]

use crate::types::{
    calculation::DoseWarning,
    drug::Drug,
    patient::{HepaticFunction, RenalFunction},
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Result of a renal or hepatic dose adjustment.
pub struct AdjustmentResult {
    pub adjusted_dose_mg: Decimal,
    pub warnings: Vec<DoseWarning>,
    pub adjustment_note: Option<String>,
}

/// Apply renal adjustment based on drug-specific data
/// Returns adjusted dose and warnings
///
/// Renal Adjustment Table (from DRUG_DATABASE.md):
/// Drug                | Normal | GFR 30-59  | GFR 15-29   | GFR < 15 / Dialysis
/// Amoxicillin         | 100%   | 100%       | Q12H        | 500mg Q24H
/// Co-amoxiclav        | 100%   | 100%       | Q12H        | Avoid
/// Co-trimoxazole      | 100%   | Reduce 50% | Avoid       | Avoid
/// Cetirizine          | 100%   | 50%        | 50% Q48H    | Avoid
/// Metronidazole       | 100%   | 100%       | Use caution | Reduce 50%
/// Oseltamivir         | 100%   | 100%       | 30mg BD     | 30mg OD
pub fn apply_renal_adjustment(
    dose_mg: Decimal,
    drug: &Drug,
    renal_function: &RenalFunction,
) -> AdjustmentResult {
    if !drug.requires_renal_adjustment {
        return AdjustmentResult {
            adjusted_dose_mg: dose_mg,
            warnings: Vec::new(),
            adjustment_note: None,
        };
    }

    match renal_function {
        RenalFunction::Normal | RenalFunction::MildImpairment => AdjustmentResult {
            adjusted_dose_mg: dose_mg,
            warnings: Vec::new(),
            adjustment_note: None,
        },
        RenalFunction::ModerateImpairment => {
            // Drug-specific moderate impairment adjustments
            let (adj_dose, note) = match drug.id.as_str() {
                "co-trimoxazole-oral" => (
                    dose_mg * dec!(0.5),
                    Some("ลด dose 50% (eGFR 30–59)".to_string()),
                ),
                "cetirizine-oral" => (
                    dose_mg * dec!(0.5),
                    Some("ลด dose 50% (eGFR 30–59)".to_string()),
                ),
                _ => {
                    // Amoxicillin, Co-amoxiclav, Metronidazole, Oseltamivir: 100%
                    (dose_mg, None)
                }
            };
            AdjustmentResult {
                adjusted_dose_mg: adj_dose,
                warnings: vec![DoseWarning::RenalAdjustmentNeeded],
                adjustment_note: note,
            }
        }
        RenalFunction::SevereImpairment => {
            let (adj_dose, note) = match drug.id.as_str() {
                "amoxicillin-oral" => (
                    dose_mg,
                    Some("ปรับ interval เป็น Q12H (eGFR 15–29)".to_string()),
                ),
                "co-amoxiclav-oral" => (
                    dose_mg,
                    Some("ปรับ interval เป็น Q12H (eGFR 15–29)".to_string()),
                ),
                "co-trimoxazole-oral" => {
                    (Decimal::ZERO, Some("หลีกเลี่ยงการใช้ (eGFR 15–29)".to_string()))
                }
                "cetirizine-oral" => (
                    dose_mg * dec!(0.5),
                    Some("ลด dose 50% และปรับ interval Q48H (eGFR 15–29)".to_string()),
                ),
                "metronidazole-oral" => (dose_mg, Some("ใช้ด้วยความระวัง (eGFR 15–29)".to_string())),
                "oseltamivir-oral" => (dec!(30), Some("ใช้ 30 mg BD (eGFR 15–29)".to_string())),
                _ => (dose_mg, None),
            };
            AdjustmentResult {
                adjusted_dose_mg: adj_dose,
                warnings: vec![DoseWarning::RenalAdjustmentNeeded],
                adjustment_note: note,
            }
        }
        RenalFunction::EndStageRenalDisease => {
            let (adj_dose, note) = match drug.id.as_str() {
                "amoxicillin-oral" => (
                    dec!(500),
                    Some("ใช้ 500 mg Q24H (ESRD/Dialysis)".to_string()),
                ),
                "co-amoxiclav-oral" => (
                    Decimal::ZERO,
                    Some("หลีกเลี่ยงการใช้ (ESRD/Dialysis)".to_string()),
                ),
                "co-trimoxazole-oral" => (
                    Decimal::ZERO,
                    Some("หลีกเลี่ยงการใช้ (ESRD/Dialysis)".to_string()),
                ),
                "cetirizine-oral" => (
                    Decimal::ZERO,
                    Some("หลีกเลี่ยงการใช้ (ESRD/Dialysis)".to_string()),
                ),
                "metronidazole-oral" => (
                    dose_mg * dec!(0.5),
                    Some("ลด dose 50% (ESRD/Dialysis)".to_string()),
                ),
                "oseltamivir-oral" => (dec!(30), Some("ใช้ 30 mg OD (ESRD/Dialysis)".to_string())),
                _ => (dose_mg, None),
            };
            AdjustmentResult {
                adjusted_dose_mg: adj_dose,
                warnings: vec![DoseWarning::RenalAdjustmentNeeded],
                adjustment_note: note,
            }
        }
    }
}

/// Apply hepatic adjustment
pub fn apply_hepatic_adjustment(
    dose_mg: Decimal,
    drug: &Drug,
    hepatic_function: &HepaticFunction,
) -> AdjustmentResult {
    if !drug.requires_hepatic_adjustment {
        return AdjustmentResult {
            adjusted_dose_mg: dose_mg,
            warnings: Vec::new(),
            adjustment_note: None,
        };
    }

    match hepatic_function {
        HepaticFunction::Normal => AdjustmentResult {
            adjusted_dose_mg: dose_mg,
            warnings: Vec::new(),
            adjustment_note: None,
        },
        HepaticFunction::MildImpairment | HepaticFunction::ModerateImpairment => {
            let note = match drug.id.as_str() {
                "metronidazole-oral" => Some("ใช้ด้วยความระวังในภาวะตับบกพร่อง".to_string()),
                "azithromycin-oral" => Some("ใช้ด้วยความระวัง — หลีกเลี่ยงในตับบกพร่องรุนแรง".to_string()),
                _ => Some("ปรึกษาเภสัชกรสำหรับการปรับขนาดยา".to_string()),
            };
            AdjustmentResult {
                adjusted_dose_mg: dose_mg,
                warnings: vec![DoseWarning::HepaticAdjustmentNeeded],
                adjustment_note: note,
            }
        }
        HepaticFunction::SevereImpairment => {
            let (adj_dose, note) = match drug.id.as_str() {
                "metronidazole-oral" => {
                    // ลด 1/3 dose ใน severe hepatic impairment
                    let reduced = dose_mg / Decimal::from(3);
                    (
                        reduced,
                        Some("ลดเหลือ 1/3 dose (severe hepatic impairment)".to_string()),
                    )
                }
                "paracetamol-oral" => (
                    Decimal::ZERO,
                    Some("ห้ามใช้ใน severe hepatic impairment".to_string()),
                ),
                "azithromycin-oral" => (
                    Decimal::ZERO,
                    Some("หลีกเลี่ยงใน severe hepatic disease".to_string()),
                ),
                _ => (dose_mg, Some("ปรึกษาเภสัชกร".to_string())),
            };
            AdjustmentResult {
                adjusted_dose_mg: adj_dose,
                warnings: vec![DoseWarning::HepaticAdjustmentNeeded],
                adjustment_note: note,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::drugs::get_drug_by_id;

    #[test]
    fn test_amoxicillin_normal_renal() {
        let drug = get_drug_by_id("amoxicillin-oral").unwrap();
        let result = apply_renal_adjustment(dec!(500), &drug, &RenalFunction::Normal);
        assert_eq!(result.adjusted_dose_mg, dec!(500));
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_cotrimoxazole_moderate_renal() {
        let drug = get_drug_by_id("co-trimoxazole-oral").unwrap();
        let result = apply_renal_adjustment(dec!(160), &drug, &RenalFunction::ModerateImpairment);
        assert_eq!(result.adjusted_dose_mg, dec!(80)); // 50% reduction
    }

    #[test]
    fn test_metronidazole_severe_hepatic() {
        let drug = get_drug_by_id("metronidazole-oral").unwrap();
        let result = apply_hepatic_adjustment(dec!(300), &drug, &HepaticFunction::SevereImpairment);
        assert_eq!(result.adjusted_dose_mg, dec!(100)); // 1/3 dose
    }
}
