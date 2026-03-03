use crate::types::{drug::Frequency, patient::AgeGroup};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Severity level for clinical warnings.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WarningSeverity {
    Info = 0,
    Caution = 1,
    Warning = 2,
    Critical = 3,
}

impl WarningSeverity {
    pub fn color_class(&self) -> &str {
        match self {
            Self::Info => "bg-blue-50 border-blue-400 text-blue-800",
            Self::Caution => "bg-yellow-50 border-yellow-400 text-yellow-800",
            Self::Warning => "bg-orange-50 border-orange-400 text-orange-800",
            Self::Critical => "bg-red-50 border-red-600 text-red-900",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::Info => "ℹ️",
            Self::Caution => "⚠️",
            Self::Warning => "🔶",
            Self::Critical => "🚨",
        }
    }

    pub fn label_th(&self) -> &str {
        match self {
            Self::Info => "ข้อมูล",
            Self::Caution => "ควรระวัง",
            Self::Warning => "คำเตือน",
            Self::Critical => "อันตราย",
        }
    }
}

/// Clinical warning variants emitted during dose calculation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DoseWarning {
    ExceedsMaxSingleDose {
        calculated_mg: Decimal,
        max_allowed_mg: Decimal,
        capped_at_mg: Decimal,
    },
    ExceedsMaxDailyDose {
        calculated_mg: Decimal,
        max_allowed_mg: Decimal,
    },
    NearMaxDose {
        percentage_of_max: Decimal,
        is_single_dose: bool,
    },
    AdultDoseCapped {
        calculated_by_weight_mg: Decimal,
        adult_dose_used_mg: Decimal,
    },
    IdealBodyWeightUsed {
        actual_weight_kg: Decimal,
        ibw_used_kg: Decimal,
    },
    RenalAdjustmentNeeded,
    HepaticAdjustmentNeeded,
    NeonatalCaution {
        message: String,
    },
    PregnancyRisk {
        category: String,
        message: String,
    },
    BreastfeedingCaution {
        message: String,
    },
    OffLabelUse {
        age_group: AgeGroup,
        message: String,
    },
    DataUnavailableForAgeGroup {
        age_group: AgeGroup,
    },
    DoseRounded {
        original_mg: Decimal,
        rounded_mg: Decimal,
    },
}

impl DoseWarning {
    pub fn severity(&self) -> WarningSeverity {
        match self {
            Self::ExceedsMaxSingleDose { .. } => WarningSeverity::Critical,
            Self::ExceedsMaxDailyDose { .. } => WarningSeverity::Critical,
            Self::RenalAdjustmentNeeded => WarningSeverity::Warning,
            Self::HepaticAdjustmentNeeded => WarningSeverity::Warning,
            Self::NeonatalCaution { .. } => WarningSeverity::Warning,
            Self::PregnancyRisk { category, .. } => match category.as_str() {
                "X" => WarningSeverity::Critical,
                "D" => WarningSeverity::Warning,
                _ => WarningSeverity::Caution,
            },
            Self::BreastfeedingCaution { .. } => WarningSeverity::Caution,
            Self::OffLabelUse { .. } => WarningSeverity::Caution,
            Self::NearMaxDose {
                percentage_of_max, ..
            } => {
                if *percentage_of_max >= Decimal::from(95) {
                    WarningSeverity::Warning
                } else {
                    WarningSeverity::Caution
                }
            }
            Self::AdultDoseCapped { .. } => WarningSeverity::Info,
            Self::IdealBodyWeightUsed { .. } => WarningSeverity::Info,
            Self::DataUnavailableForAgeGroup { .. } => WarningSeverity::Warning,
            Self::DoseRounded { .. } => WarningSeverity::Info,
        }
    }

    /// Localized Thai warning message for UI display.
    pub fn message_th(&self) -> String {
        match self {
            Self::ExceedsMaxSingleDose {
                calculated_mg,
                max_allowed_mg,
                capped_at_mg,
            } => format!(
                "ขนาดยาต่อครั้งที่คำนวณได้ ({} mg) เกินขนาดสูงสุดที่แนะนำ ({} mg) — ใช้ {} mg แทน",
                calculated_mg, max_allowed_mg, capped_at_mg
            ),
            Self::ExceedsMaxDailyDose {
                calculated_mg,
                max_allowed_mg,
            } => format!(
                "ขนาดยารวมต่อวัน ({} mg/day) เกินขนาดสูงสุด ({} mg/day)",
                calculated_mg, max_allowed_mg
            ),
            Self::NearMaxDose {
                percentage_of_max,
                is_single_dose,
            } => {
                let kind = if *is_single_dose {
                    "ต่อครั้ง"
                } else {
                    "ต่อวัน"
                };
                format!(
                    "ขนาดยา{kind}ใกล้ถึงขนาดสูงสุด ({}% ของ max dose)",
                    percentage_of_max.round_dp(0)
                )
            }
            Self::RenalAdjustmentNeeded => {
                "⚠️ ยานี้ต้องปรับขนาดในผู้ป่วยที่มีภาวะไตบกพร่อง — กรุณาปรึกษาเภสัชกร".to_string()
            }
            Self::HepaticAdjustmentNeeded => {
                "⚠️ ยานี้ต้องปรับขนาดในผู้ป่วยที่มีภาวะตับบกพร่อง — กรุณาปรึกษาเภสัชกร".to_string()
            }
            Self::NeonatalCaution { message } => format!("คำเตือนสำหรับทารกแรกเกิด: {}", message),
            Self::PregnancyRisk { category, message } => {
                format!("ความเสี่ยงระหว่างตั้งครรภ์ (Category {}): {}", category, message)
            }
            Self::BreastfeedingCaution { message } => {
                format!("ข้อควรระวังสำหรับมารดาที่ให้นมบุตร: {}", message)
            }
            Self::AdultDoseCapped {
                adult_dose_used_mg, ..
            } => format!(
                "น้ำหนัก ≥ 50 kg — ใช้ขนาดยาสำหรับผู้ใหญ่ ({} mg) แทนการคำนวณตามน้ำหนัก",
                adult_dose_used_mg
            ),
            Self::OffLabelUse { message, .. } => {
                format!("การใช้ยานอกข้อบ่งชี้ (Off-label): {}", message)
            }
            Self::DataUnavailableForAgeGroup { age_group } => format!(
                "ไม่พบข้อมูลขนาดยาสำหรับกลุ่มอายุ {} ในฐานข้อมูล กรุณาปรึกษาเภสัชกร",
                age_group.display_th()
            ),
            Self::IdealBodyWeightUsed {
                actual_weight_kg,
                ibw_used_kg,
            } => format!(
                "ใช้ Ideal Body Weight ({} kg) ในการคำนวณ แทนน้ำหนักจริง ({} kg)",
                ibw_used_kg, actual_weight_kg
            ),
            Self::DoseRounded {
                original_mg,
                rounded_mg,
            } => format!("ปัดขนาดยาจาก {} mg เป็น {} mg", original_mg, rounded_mg),
        }
    }

    /// Returns `true` if this warning requires explicit user confirmation.
    #[allow(dead_code)]
    pub fn requires_confirmation(&self) -> bool {
        matches!(self.severity(), WarningSeverity::Critical)
    }
}

/// Formatted dose for UI display.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayDose {
    pub mg_string: String,
    pub ml_string: Option<String>,
    pub formulation_used: Option<String>,
    pub rounding_note: Option<String>,
}

/// Complete dose calculation result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoseResult {
    pub single_dose_mg: Decimal,
    pub daily_dose_mg: Decimal,
    pub frequency: Frequency,
    pub min_single_dose_mg: Decimal,
    pub max_single_dose_mg: Decimal,
    pub weight_used_kg: Decimal,
    pub age_group_used: AgeGroup,
    pub dose_per_kg: Decimal,
    pub warnings: Vec<DoseWarning>,
    pub display_dose: DisplayDose,
}

impl DoseResult {
    /// Returns the highest severity among all warnings, if any.
    #[allow(dead_code)]
    pub fn max_severity(&self) -> Option<WarningSeverity> {
        self.warnings.iter().map(|w| w.severity()).max()
    }

    /// Returns `true` if any warning is critical.
    pub fn has_critical_warning(&self) -> bool {
        self.warnings
            .iter()
            .any(|w| matches!(w.severity(), WarningSeverity::Critical))
    }

    /// Returns `true` if any warning requires explicit user confirmation.
    #[allow(dead_code)]
    pub fn requires_user_confirmation(&self) -> bool {
        self.warnings.iter().any(|w| w.requires_confirmation())
    }
}

/// Errors that may occur during dose calculation.
#[derive(Clone, Debug, PartialEq, Error, Serialize, Deserialize)]
pub enum DoseError {
    #[error("น้ำหนักผู้ป่วยไม่ถูกต้อง: {0}")]
    InvalidWeight(String),

    #[error("ไม่พบข้อมูลยา ID: {0}")]
    DrugNotFound(String),

    #[error("ไม่มีข้อมูลขนาดยาสำหรับกลุ่มอายุ: {0}")]
    NoDoseForAgeGroup(String),

    #[error("ยานี้มีข้อห้ามใช้: {0}")]
    AbsoluteContraindication(String),

    #[error("ข้อมูลผู้ป่วยไม่ครบถ้วน: {0}")]
    IncompletePatientData(String),

    #[error("เกิดข้อผิดพลาดทางคณิตศาสตร์: {0}")]
    ArithmeticError(String),
}
