//! Static drug database — all entries sourced from DRUG_DATABASE.md.
//!
//! Conventions:
//! - Use `rust_decimal_macros::dec!()` for all Decimal literals.
//! - Each drug must have a unique `id`, `reference`, and `last_reviewed`.
//! - Age dosings are ordered from Neonate → Adult.
#![allow(dead_code)]

use crate::types::{
    drug::{
        AgeDosing, DoseRange, Drug, DrugCategory, Formulation, Frequency, RouteOfAdministration,
        WeightBasis,
    },
    patient::AgeGroup,
};
use rust_decimal_macros::dec;

/// Returns all drugs in the database.
pub fn all_drugs() -> Vec<Drug> {
    vec![
        amoxicillin_oral(),
        co_amoxiclav_oral(),
        paracetamol_oral(),
        ibuprofen_oral(),
        cetirizine_oral(),
        azithromycin_oral(),
        oseltamivir_oral(),
        co_trimoxazole_oral(),
        metronidazole_oral(),
    ]
}

/// Look up a drug by its unique ID.
pub fn get_drug_by_id(id: &str) -> Option<Drug> {
    all_drugs().into_iter().find(|d| d.id == id)
}

/// Search drugs by a case-insensitive query string.
pub fn search_drugs(query: &str) -> Vec<Drug> {
    let drugs = all_drugs();
    if query.is_empty() {
        return drugs;
    }
    drugs
        .into_iter()
        .filter(|d| d.matches_search(query))
        .collect()
}

// ========================================================================
// 1. Amoxicillin (Oral)
// ========================================================================
fn amoxicillin_oral() -> Drug {
    Drug {
        id: "amoxicillin-oral".into(),
        generic_name: "Amoxicillin".into(),
        brand_names: vec!["Amoxil".into(), "Ospamox".into(), "Flemox".into()],
        search_aliases: vec!["Amox".into(), "Amoxil".into()],
        category: DrugCategory::Antibiotic,
        age_dosings: vec![
            // Neonate: 30 mg/kg/day ÷ 2 = 15 mg/kg/dose BD
            AgeDosing {
                age_group: AgeGroup::Neonate,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(15),
                    max_mg_per_kg: dec!(15),
                    max_single_dose_mg: Some(dec!(125)),
                    max_daily_dose_mg: Some(dec!(250)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("ทารกแรกเกิด — ใช้ dose ต่ำ ติดตามอาการใกล้ชิด".into()),
                is_off_label: false,
            },
            // Infant/Child: 25 mg/kg/dose TID
            AgeDosing {
                age_group: AgeGroup::Infant,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(25),
                    max_mg_per_kg: dec!(25),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(3000)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily, Frequency::TwiceDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Child,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(25),
                    max_mg_per_kg: dec!(40),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(3000)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily, Frequency::TwiceDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            // Adolescent
            AgeDosing {
                age_group: AgeGroup::Adolescent,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(25),
                    max_mg_per_kg: dec!(40),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(3000)),
                    adult_flat_dose_mg: Some(dec!(500)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            // Adult (weight ≥50kg): 500 mg flat dose TID
            AgeDosing {
                age_group: AgeGroup::Adult,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(3000)),
                    adult_flat_dose_mg: Some(dec!(500)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
        ],
        formulations: vec![
            Formulation {
                description: "Syrup 125mg/5mL".into(),
                mg_per_ml: dec!(25),
                is_default: true,
            },
            Formulation {
                description: "Syrup 250mg/5mL".into(),
                mg_per_ml: dec!(50),
                is_default: false,
            },
            Formulation {
                description: "Drops 100mg/mL".into(),
                mg_per_ml: dec!(100),
                is_default: false,
            },
        ],
        contraindications: vec![
            "Penicillin hypersensitivity".into(),
            "History of amoxicillin-associated jaundice or hepatic dysfunction".into(),
        ],
        requires_renal_adjustment: true,
        requires_hepatic_adjustment: false,
        pregnancy_category: Some("B".into()),
        contraindicated_in_breastfeeding: false,
        reference: "BNFC 2023-2024; Thai Pediatric Formulary 2023".into(),
        last_reviewed: "2024-01".into(),
    }
}

// ========================================================================
// 2. Co-amoxiclav (Oral)
// ========================================================================
fn co_amoxiclav_oral() -> Drug {
    Drug {
        id: "co-amoxiclav-oral".into(),
        generic_name: "Amoxicillin-Clavulanate (Co-amoxiclav)".into(),
        brand_names: vec!["Augmentin".into(), "Curam".into(), "Amoclan".into()],
        search_aliases: vec!["Co-amox".into(), "Augmentin".into()],
        category: DrugCategory::Antibiotic,
        age_dosings: vec![
            // Neonate <7 days: 30 mg/kg/day ÷ 2 = 15 mg/kg/dose BD
            AgeDosing {
                age_group: AgeGroup::Neonate,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(15),
                    max_mg_per_kg: dec!(15),
                    max_single_dose_mg: None,
                    max_daily_dose_mg: None,
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily, Frequency::ThreeTimesDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("Neonate <7 วัน: BD, 7-28 วัน: TID".into()),
                is_off_label: false,
            },
            // Infant 1-3 months: 30 mg/kg/day ÷ 3 = 10 mg/kg/dose TID
            AgeDosing {
                age_group: AgeGroup::Infant,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(10),
                    max_mg_per_kg: dec!(25),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(3000)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            // Child >3 months: 25 mg/kg/dose TID (severe: 40 mg/kg/dose)
            AgeDosing {
                age_group: AgeGroup::Child,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(25),
                    max_mg_per_kg: dec!(40),
                    max_single_dose_mg: Some(dec!(875)),
                    max_daily_dose_mg: Some(dec!(3000)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily, Frequency::TwiceDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("ใช้ formulation 7:1 (Amox:Clav) สำหรับขนาดสูง".into()),
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Adolescent,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(25),
                    max_mg_per_kg: dec!(40),
                    max_single_dose_mg: Some(dec!(875)),
                    max_daily_dose_mg: Some(dec!(3000)),
                    adult_flat_dose_mg: Some(dec!(500)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Adult,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(875)),
                    max_daily_dose_mg: Some(dec!(3000)),
                    adult_flat_dose_mg: Some(dec!(500)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
        ],
        formulations: vec![Formulation {
            description: "Syrup 125/31.25 per 5mL".into(),
            mg_per_ml: dec!(25),
            is_default: true,
        }],
        contraindications: vec![
            "Penicillin allergy".into(),
            "History of co-amoxiclav jaundice/hepatitis".into(),
        ],
        requires_renal_adjustment: true,
        requires_hepatic_adjustment: false,
        pregnancy_category: Some("B".into()),
        contraindicated_in_breastfeeding: false,
        reference: "BNFC 2023-2024; Harriet Lane 22nd ed.".into(),
        last_reviewed: "2024-01".into(),
    }
}

// ========================================================================
// 3. Paracetamol (Oral)
// ========================================================================
fn paracetamol_oral() -> Drug {
    Drug {
        id: "paracetamol-oral".into(),
        generic_name: "Paracetamol (Acetaminophen)".into(),
        brand_names: vec![
            "Tylenol".into(),
            "Tempra".into(),
            "Sara".into(),
            "Panadol".into(),
        ],
        search_aliases: vec!["PCM".into(), "Acetaminophen".into(), "Tylenol".into()],
        category: DrugCategory::Antipyretic,
        age_dosings: vec![
            // Neonate: 10–15 mg/kg Q6–8H, max 60 mg/kg/day
            AgeDosing {
                age_group: AgeGroup::Neonate,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(10),
                    max_mg_per_kg: dec!(15),
                    max_single_dose_mg: None,
                    max_daily_dose_mg: Some(dec!(60)), // 60 mg/kg/day — special: per-kg daily
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::EveryNHours(8), Frequency::EveryNHours(6)],
                default_frequency: Frequency::EveryNHours(8),
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("ลด dose ในทารกคลอดก่อนกำหนด (preterm)".into()),
                is_off_label: false,
            },
            // Infant/Child: 15 mg/kg Q4–6H (max 5 doses/day), max single 1000mg, max daily 4000mg
            AgeDosing {
                age_group: AgeGroup::Infant,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(10),
                    max_mg_per_kg: dec!(15),
                    max_single_dose_mg: Some(dec!(1000)),
                    max_daily_dose_mg: Some(dec!(4000)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::EveryNHours(6), Frequency::EveryNHours(4)],
                default_frequency: Frequency::EveryNHours(6),
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Child,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(10),
                    max_mg_per_kg: dec!(15),
                    max_single_dose_mg: Some(dec!(1000)),
                    max_daily_dose_mg: Some(dec!(4000)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::EveryNHours(6), Frequency::EveryNHours(4)],
                default_frequency: Frequency::EveryNHours(6),
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Adolescent,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(10),
                    max_mg_per_kg: dec!(15),
                    max_single_dose_mg: Some(dec!(1000)),
                    max_daily_dose_mg: Some(dec!(4000)),
                    adult_flat_dose_mg: Some(dec!(500)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::EveryNHours(6), Frequency::EveryNHours(4)],
                default_frequency: Frequency::EveryNHours(6),
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            // Adult: 500–1000mg flat Q4–6H
            AgeDosing {
                age_group: AgeGroup::Adult,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(1000)),
                    max_daily_dose_mg: Some(dec!(4000)),
                    adult_flat_dose_mg: Some(dec!(500)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::EveryNHours(6), Frequency::EveryNHours(4)],
                default_frequency: Frequency::EveryNHours(6),
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
        ],
        formulations: vec![
            Formulation {
                description: "Syrup 120mg/5mL".into(),
                mg_per_ml: dec!(24),
                is_default: true,
            },
            Formulation {
                description: "Syrup 160mg/5mL".into(),
                mg_per_ml: dec!(32),
                is_default: false,
            },
        ],
        contraindications: vec![
            "Severe hepatic impairment".into(),
            "Hypersensitivity to paracetamol".into(),
        ],
        requires_renal_adjustment: false,
        requires_hepatic_adjustment: true,
        pregnancy_category: Some("A".into()),
        contraindicated_in_breastfeeding: false,
        reference: "BNFC 2023-2024; Thai Pediatric Formulary 2023".into(),
        last_reviewed: "2024-01".into(),
    }
}

// ========================================================================
// 4. Ibuprofen (Oral)
// ========================================================================
fn ibuprofen_oral() -> Drug {
    Drug {
        id: "ibuprofen-oral".into(),
        generic_name: "Ibuprofen".into(),
        brand_names: vec!["Brufen".into(), "Advil".into(), "Nurofen".into()],
        search_aliases: vec!["NSAID".into(), "Brufen".into()],
        category: DrugCategory::Analgesic,
        age_dosings: vec![
            // Child ≥ 3 months: 5–10 mg/kg Q6–8H, max 400mg, max daily 1200mg
            AgeDosing {
                age_group: AgeGroup::Infant,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(5),
                    max_mg_per_kg: dec!(10),
                    max_single_dose_mg: Some(dec!(400)),
                    max_daily_dose_mg: Some(dec!(1200)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::EveryNHours(8), Frequency::EveryNHours(6)],
                default_frequency: Frequency::EveryNHours(8),
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("ห้ามใช้ใน infant < 3 เดือน".into()),
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Child,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(5),
                    max_mg_per_kg: dec!(10),
                    max_single_dose_mg: Some(dec!(400)),
                    max_daily_dose_mg: Some(dec!(1200)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::EveryNHours(8), Frequency::EveryNHours(6)],
                default_frequency: Frequency::EveryNHours(8),
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Adolescent,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(5),
                    max_mg_per_kg: dec!(10),
                    max_single_dose_mg: Some(dec!(400)),
                    max_daily_dose_mg: Some(dec!(1200)),
                    adult_flat_dose_mg: Some(dec!(400)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::EveryNHours(8), Frequency::EveryNHours(6)],
                default_frequency: Frequency::EveryNHours(8),
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            // Adult: 400mg flat Q6–8H
            AgeDosing {
                age_group: AgeGroup::Adult,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(400)),
                    max_daily_dose_mg: Some(dec!(1200)),
                    adult_flat_dose_mg: Some(dec!(400)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::EveryNHours(8), Frequency::EveryNHours(6)],
                default_frequency: Frequency::EveryNHours(8),
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
        ],
        formulations: vec![Formulation {
            description: "Syrup 100mg/5mL".into(),
            mg_per_ml: dec!(20),
            is_default: true,
        }],
        contraindications: vec![
            "Infants < 3 months".into(),
            "Chickenpox / dengue fever (risk of Reye's syndrome / bleeding)".into(),
            "NSAID-triggered asthma".into(),
            "Renal impairment (risk of AKI)".into(),
            "Active GI bleeding or peptic ulcer".into(),
        ],
        requires_renal_adjustment: false, // contraindicated, not adjusted
        requires_hepatic_adjustment: false,
        pregnancy_category: Some("C".into()),
        contraindicated_in_breastfeeding: false,
        reference: "BNFC 2023-2024".into(),
        last_reviewed: "2024-01".into(),
    }
}

// ========================================================================
// 5. Cetirizine (Oral)
// ========================================================================
fn cetirizine_oral() -> Drug {
    Drug {
        id: "cetirizine-oral".into(),
        generic_name: "Cetirizine".into(),
        brand_names: vec!["Zyrtec".into(), "Zirtec".into()],
        search_aliases: vec!["Zyrtec".into(), "Antihistamine".into()],
        category: DrugCategory::Antihistamine,
        age_dosings: vec![
            // Child 6 months–2 years: 0.25 mg/kg BD, max 2.5 mg/day
            AgeDosing {
                age_group: AgeGroup::Infant,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0.25),
                    max_mg_per_kg: dec!(0.25),
                    max_single_dose_mg: Some(dec!(1.25)),
                    max_daily_dose_mg: Some(dec!(2.5)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("สำหรับเด็ก 6 เดือน – 2 ปี".into()),
                is_off_label: false,
            },
            // Child 2–12 years: 2.5–5 mg flat dose
            AgeDosing {
                age_group: AgeGroup::Child,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(5)),
                    max_daily_dose_mg: Some(dec!(10)),
                    adult_flat_dose_mg: Some(dec!(5)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::OnceDaily, Frequency::TwiceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("2–6 ปี: 2.5 mg BD, 6–12 ปี: 5 mg OD–BD".into()),
                is_off_label: false,
            },
            // Adult/Child >12 years: 10 mg OD
            AgeDosing {
                age_group: AgeGroup::Adolescent,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(10)),
                    max_daily_dose_mg: Some(dec!(10)),
                    adult_flat_dose_mg: Some(dec!(10)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::OnceDaily],
                default_frequency: Frequency::OnceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Adult,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(10)),
                    max_daily_dose_mg: Some(dec!(10)),
                    adult_flat_dose_mg: Some(dec!(10)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::OnceDaily],
                default_frequency: Frequency::OnceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
        ],
        formulations: vec![Formulation {
            description: "Syrup 5mg/5mL".into(),
            mg_per_ml: dec!(1),
            is_default: true,
        }],
        contraindications: vec![],
        requires_renal_adjustment: true,
        requires_hepatic_adjustment: false,
        pregnancy_category: Some("B".into()),
        contraindicated_in_breastfeeding: false,
        reference: "BNFC 2023-2024".into(),
        last_reviewed: "2024-01".into(),
    }
}

// ========================================================================
// 6. Azithromycin (Oral)
// ========================================================================
fn azithromycin_oral() -> Drug {
    Drug {
        id: "azithromycin-oral".into(),
        generic_name: "Azithromycin".into(),
        brand_names: vec!["Zithromax".into()],
        search_aliases: vec!["Azith".into(), "Z-pack".into()],
        category: DrugCategory::Antibiotic,
        age_dosings: vec![
            // Infant (≥6 months): 10 mg/kg day 1, then 5 mg/kg — use 10 mg/kg as max
            AgeDosing {
                age_group: AgeGroup::Infant,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(5),
                    max_mg_per_kg: dec!(10),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(500)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::OnceDaily],
                default_frequency: Frequency::OnceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("Day 1: 10 mg/kg, Day 2–5: 5 mg/kg OD".into()),
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Child,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(5),
                    max_mg_per_kg: dec!(12),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(500)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::OnceDaily],
                default_frequency: Frequency::OnceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some(
                    "Pneumonia: 10+5 mg/kg × 5 วัน; Pharyngitis: 12 mg/kg × 5 วัน".into(),
                ),
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Adolescent,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(5),
                    max_mg_per_kg: dec!(12),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(500)),
                    adult_flat_dose_mg: Some(dec!(500)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::OnceDaily],
                default_frequency: Frequency::OnceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            // Adult: 500 mg day 1, 250 mg OD
            AgeDosing {
                age_group: AgeGroup::Adult,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(500)),
                    adult_flat_dose_mg: Some(dec!(500)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::OnceDaily],
                default_frequency: Frequency::OnceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("Day 1: 500 mg, Day 2–5: 250 mg OD".into()),
                is_off_label: false,
            },
        ],
        formulations: vec![Formulation {
            description: "Syrup 200mg/5mL".into(),
            mg_per_ml: dec!(40),
            is_default: true,
        }],
        contraindications: vec![
            "Macrolide hypersensitivity".into(),
            "QT prolongation risk".into(),
        ],
        requires_renal_adjustment: false,
        requires_hepatic_adjustment: true,
        pregnancy_category: Some("B".into()),
        contraindicated_in_breastfeeding: false,
        reference: "BNFC 2023-2024".into(),
        last_reviewed: "2024-01".into(),
    }
}

// ========================================================================
// 7. Oseltamivir / Tamiflu (Oral)
// ========================================================================
fn oseltamivir_oral() -> Drug {
    // Oseltamivir uses flat dose by weight band — implemented as age groups
    Drug {
        id: "oseltamivir-oral".into(),
        generic_name: "Oseltamivir (Tamiflu)".into(),
        brand_names: vec!["Tamiflu".into()],
        search_aliases: vec!["Tamiflu".into(), "Flu drug".into()],
        category: DrugCategory::Antiviral,
        age_dosings: vec![
            // Infant < 1 year: 3 mg/kg BD
            AgeDosing {
                age_group: AgeGroup::Infant,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(3),
                    max_mg_per_kg: dec!(3),
                    max_single_dose_mg: Some(dec!(75)),
                    max_daily_dose_mg: Some(dec!(150)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("Infant <1 ปี: 3 mg/kg BD × 5 วัน".into()),
                is_off_label: false,
            },
            // Child: flat dose by weight band
            // 15–23 kg = 45 mg, 23–40 kg = 60 mg, >40 kg = 75 mg
            // Use average approach: min 3 mg/kg, max 3 mg/kg BD with flat dose caps
            AgeDosing {
                age_group: AgeGroup::Child,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(3),
                    max_mg_per_kg: dec!(3),
                    max_single_dose_mg: Some(dec!(75)),
                    max_daily_dose_mg: Some(dec!(150)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some(
                    "Flat dose ตาม weight band: 15–23kg=45mg, 23–40kg=60mg, >40kg=75mg BD × 5 วัน"
                        .into(),
                ),
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Adolescent,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(75)),
                    max_daily_dose_mg: Some(dec!(150)),
                    adult_flat_dose_mg: Some(dec!(75)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("75 mg BD × 5 วัน".into()),
                is_off_label: false,
            },
            // Adult: 75 mg flat BD
            AgeDosing {
                age_group: AgeGroup::Adult,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(75)),
                    max_daily_dose_mg: Some(dec!(150)),
                    adult_flat_dose_mg: Some(dec!(75)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("75 mg BD × 5 วัน".into()),
                is_off_label: false,
            },
        ],
        formulations: vec![Formulation {
            description: "Suspension 60mg/5mL".into(),
            mg_per_ml: dec!(12),
            is_default: true,
        }],
        contraindications: vec![],
        requires_renal_adjustment: true,
        requires_hepatic_adjustment: false,
        pregnancy_category: Some("C".into()),
        contraindicated_in_breastfeeding: false,
        reference: "WHO influenza treatment guidelines 2022".into(),
        last_reviewed: "2024-01".into(),
    }
}

// ========================================================================
// 8. Co-trimoxazole (Oral)
// ========================================================================
fn co_trimoxazole_oral() -> Drug {
    Drug {
        id: "co-trimoxazole-oral".into(),
        generic_name: "Trimethoprim-Sulfamethoxazole (Co-trimoxazole)".into(),
        brand_names: vec!["Bactrim".into(), "Septrin".into()],
        search_aliases: vec!["Co-trim".into(), "TMP-SMX".into(), "Bactrim".into()],
        category: DrugCategory::Antibiotic,
        age_dosings: vec![
            // Child ≥ 2 months: 4–6 mg/kg/dose (TMP) BD
            AgeDosing {
                age_group: AgeGroup::Infant,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(4),
                    max_mg_per_kg: dec!(6),
                    max_single_dose_mg: Some(dec!(160)),
                    max_daily_dose_mg: Some(dec!(320)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily, Frequency::OnceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("Dose เป็น TMP component; ≥2 เดือน เท่านั้น".into()),
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Child,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(4),
                    max_mg_per_kg: dec!(6),
                    max_single_dose_mg: Some(dec!(160)),
                    max_daily_dose_mg: Some(dec!(320)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily, Frequency::OnceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("UTI: 4–6 mg/kg BD; Prophylaxis UTI: 2 mg/kg OD; PCP prophylaxis: 5 mg/kg OD 3 วัน/สัปดาห์".into()),
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Adolescent,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(4),
                    max_mg_per_kg: dec!(6),
                    max_single_dose_mg: Some(dec!(160)),
                    max_daily_dose_mg: Some(dec!(320)),
                    adult_flat_dose_mg: Some(dec!(160)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            // Adult UTI: 160 mg TMP flat BD
            AgeDosing {
                age_group: AgeGroup::Adult,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(160)),
                    max_daily_dose_mg: Some(dec!(320)),
                    adult_flat_dose_mg: Some(dec!(160)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
        ],
        formulations: vec![],
        contraindications: vec![
            "Infants < 6 weeks (risk of kernicterus)".into(),
            "G6PD deficiency".into(),
            "Sulfonamide allergy".into(),
        ],
        requires_renal_adjustment: true,
        requires_hepatic_adjustment: false,
        pregnancy_category: Some("C".into()),
        contraindicated_in_breastfeeding: false,
        reference: "BNFC 2023-2024".into(),
        last_reviewed: "2024-01".into(),
    }
}

// ========================================================================
// 9. Metronidazole (Oral)
// ========================================================================
fn metronidazole_oral() -> Drug {
    Drug {
        id: "metronidazole-oral".into(),
        generic_name: "Metronidazole".into(),
        brand_names: vec!["Flagyl".into()],
        search_aliases: vec!["Flagyl".into(), "Metro".into()],
        category: DrugCategory::Antibiotic,
        age_dosings: vec![
            // Neonate <7 days: 7.5 mg/kg BD
            AgeDosing {
                age_group: AgeGroup::Neonate,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(7.5),
                    max_mg_per_kg: dec!(7.5),
                    max_single_dose_mg: None,
                    max_daily_dose_mg: None,
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::TwiceDaily, Frequency::ThreeTimesDaily],
                default_frequency: Frequency::TwiceDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("Neonate <7 วัน: BD, 7-28 วัน: TID".into()),
                is_off_label: false,
            },
            // Infant/Child: 7.5 mg/kg TID, max single 400mg
            AgeDosing {
                age_group: AgeGroup::Infant,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(5),
                    max_mg_per_kg: dec!(10),
                    max_single_dose_mg: Some(dec!(400)),
                    max_daily_dose_mg: Some(dec!(1200)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily, Frequency::TwiceDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Child,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(5),
                    max_mg_per_kg: dec!(10),
                    max_single_dose_mg: Some(dec!(400)),
                    max_daily_dose_mg: Some(dec!(1200)),
                    adult_flat_dose_mg: None,
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily, Frequency::TwiceDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: Some("Anaerobic: 7.5 mg/kg TID; H. pylori: 7.5–10 mg/kg BD; Giardia/Amoeba: 5 mg/kg TID × 5 วัน".into()),
                is_off_label: false,
            },
            AgeDosing {
                age_group: AgeGroup::Adolescent,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(5),
                    max_mg_per_kg: dec!(10),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(4000)),
                    adult_flat_dose_mg: Some(dec!(400)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
            // Adult: 400–500 mg flat TID
            AgeDosing {
                age_group: AgeGroup::Adult,
                dose_range: DoseRange {
                    min_mg_per_kg: dec!(0),
                    max_mg_per_kg: dec!(0),
                    max_single_dose_mg: Some(dec!(500)),
                    max_daily_dose_mg: Some(dec!(4000)),
                    adult_flat_dose_mg: Some(dec!(400)),
                    weight_basis: WeightBasis::Actual,
                },
                available_frequencies: vec![Frequency::ThreeTimesDaily],
                default_frequency: Frequency::ThreeTimesDaily,
                route: RouteOfAdministration::Oral,
                clinical_notes: None,
                is_off_label: false,
            },
        ],
        formulations: vec![
            Formulation { description: "Syrup 200mg/5mL".into(), mg_per_ml: dec!(40), is_default: true },
        ],
        contraindications: vec![],
        requires_renal_adjustment: true,
        requires_hepatic_adjustment: true,
        pregnancy_category: Some("B".into()),
        contraindicated_in_breastfeeding: false,
        reference: "BNFC 2023-2024; Thai Pediatric Formulary 2023".into(),
        last_reviewed: "2024-01".into(),
    }
}

// ===== Tests =====
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_drugs_loaded() {
        let drugs = all_drugs();
        assert_eq!(drugs.len(), 9);
    }

    #[test]
    fn test_get_drug_by_id() {
        let drug = get_drug_by_id("amoxicillin-oral");
        assert!(drug.is_some());
        assert_eq!(drug.unwrap().generic_name, "Amoxicillin");
    }

    #[test]
    fn test_search_finds_by_generic() {
        let results = search_drugs("amox");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_finds_by_brand() {
        let results = search_drugs("Augmentin");
        assert!(!results.is_empty());
        assert!(results.iter().any(|d| d.id == "co-amoxiclav-oral"));
    }

    #[test]
    fn test_search_finds_by_alias() {
        let results = search_drugs("PCM");
        assert!(!results.is_empty());
        assert!(results.iter().any(|d| d.id == "paracetamol-oral"));
    }

    #[test]
    fn test_each_drug_has_age_dosings() {
        for drug in all_drugs() {
            assert!(
                !drug.age_dosings.is_empty(),
                "Drug {} has no age_dosings",
                drug.id
            );
        }
    }

    #[test]
    fn test_unique_drug_ids() {
        let drugs = all_drugs();
        let mut ids: Vec<&str> = drugs.iter().map(|d| d.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), drugs.len(), "Duplicate drug IDs found");
    }
}
