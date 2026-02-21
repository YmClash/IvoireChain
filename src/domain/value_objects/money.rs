// //! Money value object - represents monetary amounts in FCFA centimes.
//
// use serde::{Deserialize, Serialize};
// use std::fmt;
//
// /// Money value object storing amounts in centimes.
// /// Using i64 to avoid floating-point precision issues.
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct Money {
//     /// Amount in centimes (1 FCFA = 100 centimes)
//     centimes: i64,
// }
//
//
// impl Money{
//     /// Create Money from centimes
//     #[inline]
//     pub fn from_centimes(centimes: i64) -> Self {
//         Self { centimes }
//     }
//
//     /// Create Money from FCFA
//     #[inline]
//     pub fn from_fcfa(fcfa: f64) -> Self {
//         Self {
//             centimes: (fcfa * 100.0).round() as i64,
//         }
//     }
//
//     /// Create zero Money
//     #[inline]
//     pub fn zero() -> Self {
//         Self { centimes: 0 }
//     }
//
//     /// Get amount in centimes
//     #[inline]
//     pub fn centimes(&self) -> i64 {
//         self.centimes
//     }
//
//     /// Get amount in FCFA
//     #[inline]
//     pub fn to_fcfa(&self) -> f64 {
//         self.centimes as f64 / 100.0
//     }
//
//     /// Check if amount is zero
//     #[inline]
//     pub fn is_zero(&self) -> bool {
//         self.centimes == 0
//     }
//
//     /// Check if amount is positive
//     #[inline]
//     pub fn is_positive(&self) -> bool {
//         self.centimes > 0
//     }
//
//     /// Add two Money amounts
//     pub fn add(&self, other: &Money) -> Money {
//         Money::from_centimes(self.centimes + other.centimes)
//     }
//
//     /// Subtract Money (returns None if result would be negative)
//     pub fn subtract(&self, other: &Money) -> Option<Money> {
//         if self.centimes >= other.centimes {
//             Some(Money::from_centimes(self.centimes - other.centimes))
//         } else {
//             None
//         }
//     }
// }
//
// impl fmt::Display for Money {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:.2} FCFA", self.to_fcfa())
//     }
// }
//
// impl Default for Money {
//     fn default() -> Self {
//         Self::zero()
//     }
// }
//
//
