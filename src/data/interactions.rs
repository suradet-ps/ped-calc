//! Drug interaction data (placeholder for future expansion).
#![allow(dead_code)]

use crate::types::calculation::WarningSeverity;

/// Drug interaction entry
pub struct DrugInteraction {
    pub drug_a_id: &'static str,
    pub drug_b_id: &'static str,
    pub severity: WarningSeverity,
    pub description_th: &'static str,
    pub management_th: &'static str,
}

/// Known drug interactions (future expansion)
pub static DRUG_INTERACTIONS: &[DrugInteraction] = &[];
