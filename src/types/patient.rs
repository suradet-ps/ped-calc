use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Patient age group used to select the appropriate dosing protocol.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AgeGroup {
    Neonate,
    Infant,
    Child,
    Adolescent,
    Adult,
}

impl AgeGroup {
    pub fn display_th(&self) -> &str {
        match self {
            Self::Neonate => "ทารกแรกเกิด (0–28 วัน)",
            Self::Infant => "ทารก (1–12 เดือน)",
            Self::Child => "เด็ก (1–12 ปี)",
            Self::Adolescent => "วัยรุ่น (13–17 ปี)",
            Self::Adult => "ผู้ใหญ่ (≥18 ปี)",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RenalFunction {
    Normal,
    MildImpairment,
    ModerateImpairment,
    SevereImpairment,
    EndStageRenalDisease,
}

impl RenalFunction {
    #[allow(dead_code)]
    pub fn display_th(&self) -> &str {
        match self {
            Self::Normal => "ปกติ",
            Self::MildImpairment => "บกพร่องเล็กน้อย (eGFR 60–89)",
            Self::ModerateImpairment => "บกพร่องปานกลาง (eGFR 30–59)",
            Self::SevereImpairment => "บกพร่องรุนแรง (eGFR 15–29)",
            Self::EndStageRenalDisease => "ไตวายเรื้อรังระยะสุดท้าย / ฟอกไต",
        }
    }

    pub fn is_impaired(&self) -> bool {
        !matches!(self, Self::Normal)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HepaticFunction {
    Normal,
    MildImpairment,
    ModerateImpairment,
    SevereImpairment,
}

impl HepaticFunction {
    #[allow(dead_code)]
    pub fn display_th(&self) -> &str {
        match self {
            Self::Normal => "ปกติ",
            Self::MildImpairment => "บกพร่องเล็กน้อย (Child-Pugh A)",
            Self::ModerateImpairment => "บกพร่องปานกลาง (Child-Pugh B)",
            Self::SevereImpairment => "บกพร่องรุนแรง (Child-Pugh C)",
        }
    }

    pub fn is_impaired(&self) -> bool {
        !matches!(self, Self::Normal)
    }
}

/// Patient data required for dose calculation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatientData {
    pub weight_kg: Decimal,
    pub age_years: Option<u8>,
    pub age_months: Option<u8>,
    pub age_days: Option<u16>,
    pub sex: Option<Sex>,
    pub renal_function: RenalFunction,
    pub hepatic_function: HepaticFunction,
    pub is_pregnant: bool,
    pub is_breastfeeding: bool,
}

impl Default for PatientData {
    fn default() -> Self {
        Self {
            weight_kg: Decimal::ZERO,
            age_years: None,
            age_months: None,
            age_days: None,
            sex: None,
            renal_function: RenalFunction::Normal,
            hepatic_function: HepaticFunction::Normal,
            is_pregnant: false,
            is_breastfeeding: false,
        }
    }
}

impl PatientData {
    /// Determine the patient's age group from the provided age fields.
    pub fn age_group(&self) -> AgeGroup {
        if let Some(days) = self.age_days {
            if days <= 28 {
                return AgeGroup::Neonate;
            }
        }
        if let Some(months) = self.age_months {
            if months < 12 {
                return AgeGroup::Infant;
            }
        }
        match self.age_years {
            None => AgeGroup::Adult,
            Some(0) => AgeGroup::Infant,
            Some(y) if y <= 12 => AgeGroup::Child,
            Some(y) if y <= 17 => AgeGroup::Adolescent,
            _ => AgeGroup::Adult,
        }
    }

    /// Estimate ideal body weight using the Traub formula.
    pub fn ideal_body_weight_kg(&self) -> Option<Decimal> {
        let years = self.age_years? as i32;
        if years < 1 {
            return None;
        }
        let ibw = match self.sex {
            Some(Sex::Female) => Decimal::new(25, 1) * Decimal::from(years) + Decimal::from(7),
            _ => Decimal::new(25, 1) * Decimal::from(years) + Decimal::from(9),
        };
        Some(ibw)
    }

    /// Validate that sufficient data is provided for dose calculation.
    pub fn is_valid_for_calculation(&self) -> Result<(), String> {
        if self.weight_kg <= Decimal::ZERO {
            return Err("กรุณากรอกน้ำหนักผู้ป่วย".to_string());
        }
        if self.weight_kg > Decimal::from(300) {
            return Err("น้ำหนักผู้ป่วยเกินค่าสูงสุดที่รองรับ (300 kg)".to_string());
        }
        Ok(())
    }
}
