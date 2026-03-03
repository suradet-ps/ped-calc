use crate::types::patient::AgeGroup;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Primary drug category.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DrugCategory {
    Antibiotic,
    Analgesic,
    Antipyretic,
    Antiviral,
    Antifungal,
    Antiparasitic,
    Cardiovascular,
    Respiratory,
    Gastrointestinal,
    Neurological,
    Endocrine,
    Antihistamine,
    Corticosteroid,
    Other(String),
}

impl DrugCategory {
    /// Thai display label for the UI.
    pub fn display_th(&self) -> &str {
        match self {
            Self::Antibiotic => "ยาปฏิชีวนะ",
            Self::Analgesic => "ยาแก้ปวด",
            Self::Antipyretic => "ยาลดไข้",
            Self::Antiviral => "ยาต้านไวรัส",
            Self::Antifungal => "ยาต้านเชื้อรา",
            Self::Antiparasitic => "ยาต้านปรสิต",
            Self::Cardiovascular => "ยาหัวใจและหลอดเลือด",
            Self::Respiratory => "ยาระบบทางเดินหายใจ",
            Self::Gastrointestinal => "ยาระบบทางเดินอาหาร",
            Self::Neurological => "ยาระบบประสาท",
            Self::Endocrine => "ยาต่อมไร้ท่อ",
            Self::Antihistamine => "ยาแก้แพ้",
            Self::Corticosteroid => "ยาสเตียรอยด์",
            Self::Other(s) => s.as_str(),
        }
    }

    /// English display label.
    pub fn display_en(&self) -> &str {
        match self {
            Self::Antibiotic => "Antibiotic",
            Self::Analgesic => "Analgesic",
            Self::Antipyretic => "Antipyretic",
            Self::Antiviral => "Antiviral",
            Self::Antifungal => "Antifungal",
            Self::Antiparasitic => "Antiparasitic",
            Self::Cardiovascular => "Cardiovascular",
            Self::Respiratory => "Respiratory",
            Self::Gastrointestinal => "GI",
            Self::Neurological => "Neurological",
            Self::Endocrine => "Endocrine",
            Self::Antihistamine => "Antihistamine",
            Self::Corticosteroid => "Corticosteroid",
            Self::Other(s) => s.as_str(),
        }
    }
}

/// Route of drug administration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RouteOfAdministration {
    Oral,
    Intravenous,
    Intramuscular,
    Subcutaneous,
    Topical,
    Inhalation,
    Rectal,
    Sublingual,
    Intranasal,
    Ophthalmic,
}

impl RouteOfAdministration {
    pub fn abbreviation(&self) -> &str {
        match self {
            Self::Oral => "PO",
            Self::Intravenous => "IV",
            Self::Intramuscular => "IM",
            Self::Subcutaneous => "SC",
            Self::Topical => "Top",
            Self::Inhalation => "Inh",
            Self::Rectal => "PR",
            Self::Sublingual => "SL",
            Self::Intranasal => "IN",
            Self::Ophthalmic => "Ophth",
        }
    }

    /// Thai display label for the UI.
    pub fn display_th(&self) -> &str {
        match self {
            Self::Oral => "กินทางปาก",
            Self::Intravenous => "ฉีดเข้าหลอดเลือดดำ",
            Self::Intramuscular => "ฉีดเข้ากล้ามเนื้อ",
            Self::Subcutaneous => "ฉีดเข้าใต้ผิวหนัง",
            Self::Topical => "ทาภายนอก",
            Self::Inhalation => "สูดพ่น",
            Self::Rectal => "ทางทวารหนัก",
            Self::Sublingual => "อมใต้ลิ้น",
            Self::Intranasal => "พ่นจมูก",
            Self::Ophthalmic => "หยอดตา",
        }
    }
}

/// Dosing frequency.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Frequency {
    OnceDaily,
    TwiceDaily,
    ThreeTimesDaily,
    FourTimesDaily,
    EveryNHours(u8),
    AsNeeded,
    Custom(String),
}

impl Frequency {
    pub fn abbreviation(&self) -> String {
        match self {
            Self::OnceDaily => "OD".to_string(),
            Self::TwiceDaily => "BD".to_string(),
            Self::ThreeTimesDaily => "TID".to_string(),
            Self::FourTimesDaily => "QID".to_string(),
            Self::EveryNHours(n) => format!("Q{}H", n),
            Self::AsNeeded => "PRN".to_string(),
            Self::Custom(s) => s.clone(),
        }
    }

    pub fn doses_per_day(&self) -> Option<Decimal> {
        match self {
            Self::OnceDaily => Some(Decimal::ONE),
            Self::TwiceDaily => Some(Decimal::from(2)),
            Self::ThreeTimesDaily => Some(Decimal::from(3)),
            Self::FourTimesDaily => Some(Decimal::from(4)),
            Self::EveryNHours(n) => {
                if *n == 0 {
                    return None;
                }
                Some(Decimal::from(24) / Decimal::from(*n))
            }
            Self::AsNeeded => None,
            Self::Custom(_) => None,
        }
    }

    /// Thai display label for the UI.
    pub fn display_th(&self) -> String {
        match self {
            Self::OnceDaily => "วันละ 1 ครั้ง".to_string(),
            Self::TwiceDaily => "วันละ 2 ครั้ง (ทุก 12 ชั่วโมง)".to_string(),
            Self::ThreeTimesDaily => "วันละ 3 ครั้ง (ทุก 8 ชั่วโมง)".to_string(),
            Self::FourTimesDaily => "วันละ 4 ครั้ง (ทุก 6 ชั่วโมง)".to_string(),
            Self::EveryNHours(n) => format!("ทุก {} ชั่วโมง", n),
            Self::AsNeeded => "เมื่อจำเป็น (PRN)".to_string(),
            Self::Custom(s) => s.clone(),
        }
    }
}

/// Body weight selection strategy for dosing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum WeightBasis {
    Actual,
    Ideal,
    Adjusted,
    Lean,
}

/// Dose range for a specific age group and indication.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoseRange {
    pub min_mg_per_kg: Decimal,
    pub max_mg_per_kg: Decimal,
    pub max_single_dose_mg: Option<Decimal>,
    pub max_daily_dose_mg: Option<Decimal>,
    pub adult_flat_dose_mg: Option<Decimal>,
    pub weight_basis: WeightBasis,
}

/// Age-specific dosing protocol.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgeDosing {
    pub age_group: AgeGroup,
    pub dose_range: DoseRange,
    pub available_frequencies: Vec<Frequency>,
    pub default_frequency: Frequency,
    pub route: RouteOfAdministration,
    pub clinical_notes: Option<String>,
    pub is_off_label: bool,
}

/// Commercial formulation for mg → mL conversion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Formulation {
    pub description: String,
    pub mg_per_ml: Decimal,
    pub is_default: bool,
}

/// Complete drug entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Drug {
    pub id: String,
    pub generic_name: String,
    pub brand_names: Vec<String>,
    pub search_aliases: Vec<String>,
    pub category: DrugCategory,
    pub age_dosings: Vec<AgeDosing>,
    pub formulations: Vec<Formulation>,
    pub contraindications: Vec<String>,
    pub requires_renal_adjustment: bool,
    pub requires_hepatic_adjustment: bool,
    pub pregnancy_category: Option<String>,
    pub contraindicated_in_breastfeeding: bool,
    pub reference: String,
    pub last_reviewed: String,
}

impl Drug {
    /// Find the dosing protocol matching the given age group.
    pub fn find_dosing_for_age(&self, age_group: &AgeGroup) -> Option<&AgeDosing> {
        self.age_dosings.iter().find(|d| &d.age_group == age_group)
    }

    /// Display name combining generic name and first brand name.
    pub fn display_name(&self) -> String {
        if let Some(brand) = self.brand_names.first() {
            format!("{} ({})", self.generic_name, brand)
        } else {
            self.generic_name.clone()
        }
    }

    /// Returns `true` if the drug matches a case-insensitive search query.
    pub fn matches_search(&self, query: &str) -> bool {
        let q = query.to_lowercase();
        self.generic_name.to_lowercase().contains(&q)
            || self
                .brand_names
                .iter()
                .any(|b| b.to_lowercase().contains(&q))
            || self
                .search_aliases
                .iter()
                .any(|a| a.to_lowercase().contains(&q))
    }
}
