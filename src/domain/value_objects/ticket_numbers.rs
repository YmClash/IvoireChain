//! TicketNumbers value object - validated lottery number selection.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::domain::entities::Ticket;
use crate::shared::errors::DomainError;

/// Default lottery configuration
const DEFAULT_MIN_NUMBER:i32 = 1;
const DEFAULT_MAX_NUMBER:i32 = 49;
const DEFAULT_COUNT:usize = 6;

/// TicketNumbers value object - immutable validated lottery number
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketNumbers{
    /// Sorted list of selected numbers
    numbers: Vec<i32>,
}

impl TicketNumbers{
    /// Create and validate Ticker numbers with default range (1-49,6 numbers)
    pub fn new(numbers: Vec<i32>) -> Result<Self, DomainError> {
        Self::with_config(
            numbers,
            DEFAULT_MIN_NUMBER,
            DEFAULT_MAX_NUMBER,
            DEFAULT_COUNT,
        )
    }

    ///Create and validate ticket numbers with custom configuration.
    pub fn with_config(
        numbers: Vec<i32>,
        min: i32,
        max: i32,
        count: usize,
    ) -> Result<Self, DomainError> {
        // Validate count
        if numbers.len() != count {
            return Err(DomainError::InvalidTicketNumbers(format!(
                "Vous devez sélectionner exactement {} numéros, {} fournis",
                count,
                numbers.len()
            )));
        }

        // Check for duplicates
        let mut seen = HashSet::new();
        for &num in &numbers {
            if !seen.insert(num) {
                return Err(DomainError::InvalidTicketNumbers(format!(
                    "Numéro en double: {}",
                    num
                )));
            }
        }

        // Validate range
        for &num in &numbers {
            if num < min || num > max {
                return Err(DomainError::InvalidTicketNumbers(format!(
                    "Le numéro {} est hors de la plage {}-{}",
                    num, min, max
                )));
            }
        }

        // Sort numbers for consistent storage
        let mut sorted = numbers;
        sorted.sort();

        Ok(Self { numbers: sorted })
    }


    /// Get the numbers as a slice
    pub fn as_slice(&self) -> &[i32] {
        &self.numbers
    }

    /// Get the numbers as a vector (cloned)
    pub fn to_vec(&self) -> Vec<i32> {
        self.numbers.clone()
    }

    /// Check if a specific number is in the selection
    pub fn contains(&self, number: i32) -> bool {
        self.numbers.contains(&number)
    }

    /// Count matching numbers with another set
    pub fn count_matches(&self, other: &[i32]) -> usize {
        self.numbers.iter().filter(|n| other.contains(n)).count()
    }

    /// Get matching numbers with another set
    pub fn get_matches(&self, other: &[i32]) -> Vec<i32> {
        self.numbers
            .iter()
            .filter(|n| other.contains(n))
            .copied()
            .collect()
    }
}


impl std::fmt::Display for TicketNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nums: Vec<String> = self.numbers.iter().map(|n| n.to_string()).collect();
        write!(f, "[{}]", nums.join(", "))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_numbers() {
        let nums = TicketNumbers::new(vec![1, 15, 23, 34, 42, 49]);
        assert!(nums.is_ok());

        let nums = nums.unwrap();
        assert_eq!(nums.as_slice(), &[1, 15, 23, 34, 42, 49]);
    }

    #[test]
    fn test_numbers_are_sorted() {
        let nums = TicketNumbers::new(vec![49, 1, 23, 15, 42, 34]).unwrap();
        assert_eq!(nums.as_slice(), &[1, 15, 23, 34, 42, 49]);
    }

    #[test]
    fn test_invalid_count() {
        let nums = TicketNumbers::new(vec![1, 2, 3]);
        assert!(nums.is_err());
    }

    #[test]
    fn test_duplicates() {
        let nums = TicketNumbers::new(vec![1, 1, 23, 34, 42, 49]);
        assert!(nums.is_err());
    }

    #[test]
    fn test_out_of_range() {
        let nums = TicketNumbers::new(vec![0, 15, 23, 34, 42, 49]);
        assert!(nums.is_err());

        let nums = TicketNumbers::new(vec![1, 15, 23, 34, 42, 50]);
        assert!(nums.is_err());
    }

    #[test]
    fn test_count_matches() {
        let nums = TicketNumbers::new(vec![1, 15, 23, 34, 42, 49]).unwrap();
        let winning = vec![1, 23, 42, 7, 8, 9];

        assert_eq!(nums.count_matches(&winning), 3);
        assert_eq!(nums.get_matches(&winning), vec![1, 23, 42]);
    }
}
