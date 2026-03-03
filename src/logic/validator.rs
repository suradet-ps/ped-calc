//! Input validation — pure functions for patient data.
#![allow(dead_code)]

use crate::types::patient::PatientData;
use rust_decimal::Decimal;

/// Validate patient weight (kg).
pub fn validate_weight(weight_kg: Decimal) -> Result<(), String> {
    if weight_kg <= Decimal::ZERO {
        return Err("น้ำหนักต้องมากกว่า 0 kg".to_string());
    }
    if weight_kg < Decimal::new(1, 1) {
        return Err("น้ำหนักต้องไม่น้อยกว่า 0.1 kg".to_string());
    }
    if weight_kg > Decimal::from(200) {
        return Err("น้ำหนักสูงสุดที่รองรับคือ 200 kg".to_string());
    }
    Ok(())
}

/// Validate age in years.
pub fn validate_age_years(years: u8) -> Result<(), String> {
    if years > 120 {
        return Err("อายุไม่ถูกต้อง".to_string());
    }
    Ok(())
}

/// Validate age in days (neonatal range).
pub fn validate_age_days(days: u16) -> Result<(), String> {
    if days > 28 {
        return Err("ทารกแรกเกิดต้องมีอายุไม่เกิน 28 วัน — กรุณาใช้โหมดเดือนแทน".to_string());
    }
    Ok(())
}

/// Validate age in months.
pub fn validate_age_months(months: u8) -> Result<(), String> {
    if months > 23 {
        return Err("กรุณาใช้โหมดปีสำหรับอายุ 2 ปีขึ้นไป".to_string());
    }
    Ok(())
}

/// Validate all patient data fields and return a list of errors.
pub fn validate_patient_data(patient: &PatientData) -> Vec<String> {
    let mut errors = Vec::new();

    if let Err(e) = validate_weight(patient.weight_kg) {
        errors.push(e);
    }

    if let Some(days) = patient.age_days {
        if let Err(e) = validate_age_days(days) {
            errors.push(e);
        }
    }

    if let Some(months) = patient.age_months {
        if let Err(e) = validate_age_months(months) {
            errors.push(e);
        }
    }

    if let Some(years) = patient.age_years {
        if let Err(e) = validate_age_years(years) {
            errors.push(e);
        }
    }

    errors
}
