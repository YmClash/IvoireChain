//! Utility functions and helpers.

use chrono::{DateTime, Utc};

/// Convert centime to FCFA (Divide by 100)
#[inline]
pub fn centimes_to_fcfa(centimes: i64) -> f64 {
    centimes as f64  / 100.0
}

/// Convert FCFA to centime (Multiply by 100)
#[inline]
pub fn fcfa_to_centimes(fcfa: f64) -> i64 {
    (fcfa *100.0).round() as i64
}

/// Get current UTC timestamp
#[inline]
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Validate lottery numbers are within valid range
pub fn validate_lottery_numbers(
    numbers: &[i32],
    min: i32,
    max: i32,
    count: usize,
) -> Result<(), String> {
    if numbers.len() != count {
        return Err(format!(
            "Vous devez sélectionner exactement {} numéros",
            count
        ));
    }

    let mut seen = std::collections::HashSet::new();
    for &num in numbers {
        if num < min || num > max {
            return Err(format!("Les numéros doivent être entre {} et {}", min, max));
        }
        if !seen.insert(num) {
            return Err("Les numéros doivent être uniques".to_string());
        }
    }

    Ok(())
}

/// Mask phone number for display (e.g., +225 07 ** ** 45 67)
pub fn mask_phone_number(phone: &str) -> String {
    if phone.len() < 8 {
        return phone.to_string();
    }

    let visible_start = 4.min(phone.len() / 3);
    let visible_end = 4.min(phone.len() / 3);
    let masked_len = phone.len().saturating_sub(visible_start + visible_end);

    format!(
        "{}{}{}",
        &phone[..visible_start],
        "*".repeat(masked_len),
        &phone[phone.len() - visible_end..]
    )
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centimes_conversion() {
        assert_eq!(centimes_to_fcfa(50000), 500.0);
        assert_eq!(fcfa_to_centimes(500.0), 50000);
        assert_eq!(centimes_to_fcfa(12345), 123.45);
        assert_eq!(fcfa_to_centimes(123.45), 12345);
    }

    #[test]
    fn test_validate_lottery_numbers() {
        // Valid numbers
        assert!(validate_lottery_numbers(&[1, 15, 23, 34, 42, 49], 1, 49, 6).is_ok());

        // Wrong count
        assert!(validate_lottery_numbers(&[1, 15, 23], 1, 49, 6).is_err());

        // Out of range
        assert!(validate_lottery_numbers(&[0, 15, 23, 34, 42, 49], 1, 49, 6).is_err());
        assert!(validate_lottery_numbers(&[1, 15, 23, 34, 42, 50], 1, 49, 6).is_err());

        // Duplicates
        assert!(validate_lottery_numbers(&[1, 1, 23, 34, 42, 49], 1, 49, 6).is_err());
    }

    #[test]
    fn test_mask_phone_number() {
        assert_eq!(mask_phone_number("+2250701234567"), "+225******4567");
    }
}
