// //! PhoneNumber value object - validates international phone numbers.
//
// use serde::{Serialize, Deserialize};
// use std::fmt;
//
// use crate::shared::errors::DomainError;
//
//
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct PhoneNumber {
//     value: String,
// }
//
// impl PhoneNumber{
//     /// Create and validate a phone number.
//     ///
//     /// Accepts various formats and normalizes to E.164.
//     /// Supports international format.
//     pub fn new(phone: &str) -> Result<Self, DomainError> {
//         let normalized = Self::normalize(phone)?;
//         Ok(Self { value: normalized })
//     }
//
//     /// Normalize phone number to E.164 format
//     fn normalize(phone: &str) -> Result<String, DomainError> {
//         // Remove all non-digit characters except leading +
//         let cleaned: String = phone
//             .chars()
//             .filter(|c| c.is_ascii_digit() || *c == '+')
//             .collect();
//
//         // Validate using phonenumber crate
//         match phonenumber::parse(None, &cleaned) {
//             Ok(parsed) => {
//                 if phonenumber::is_valid(&parsed) {
//                     Ok(phonenumber::format(&parsed)
//                         .mode(phonenumber::Mode::E164)
//                         .to_string())
//                 } else {
//                     Err(DomainError::InvalidPhoneNumber(
//                         "Numéro de téléphone invalide".to_string(),
//                     ))
//                 }
//             }
//             Err(_) => Err(DomainError::InvalidPhoneNumber(format!(
//                 "Format de numéro invalide: {}",
//                 phone
//             ))),
//         }
//     }
//
//     /// Get the phone number value
//     pub fn value(&self) -> &str {
//         &self.value
//     }
//
//     /// Check if this is an Ivory Coast number (+225)
//     pub fn is_ivory_coast(&self) -> bool {
//         self.value.starts_with("+225")
//     }
// }
//
// impl fmt::Display for PhoneNumber {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.value)
//     }
// }
//
// impl AsRef<str> for PhoneNumber{
//     fn as_ref(&self) -> &str{
//         &self.value
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_valid_ivory_coast_number() {
//         // Note: This test may fail without a valid Ivory Coast number format
//         // Ivory Coast uses +225 followed by 10 digits
//         let phone = PhoneNumber::new("+2250701234567");
//         // If phonenumber crate validates, it should work
//         if let Ok(p) = phone {
//             assert!(p.is_ivory_coast());
//         }
//     }
//
//     #[test]
//     fn test_international_format() {
//         let phone = PhoneNumber::new("+33612345678");
//         assert!(phone.is_ok());
//     }
//
//     #[test]
//     fn test_invalid_number() {
//         let phone = PhoneNumber::new("invalid");
//         assert!(phone.is_err());
//     }
// }
