//! LotteryDraw entity representing a scheduled lottery draw.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::Money;

/// Draw Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "draw_status", rename_all = "snake_case")]
pub enum DrawStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

impl std::fmt::Display for DrawStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawStatus::Scheduled => write!(f, "scheduled"),
            DrawStatus::InProgress => write!(f, "in_progress"),
            DrawStatus::Completed => write!(f, "completed"),
            DrawStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// LotteryDraw entity representing a scheduled lottery draw.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LotteryDraw {
    /// Unique identifier
    pub id: Uuid,
    /// Name of the draw (e.g., "Tirage du Samedi")
    pub draw_name: String,
    /// Scheduled draw date
    pub draw_date: DateTime<Utc>,
    /// Winning numbers (set after draw)
    pub winning_numbers: Option<Vec<i32>>,
    /// Total prize pool
    pub prize_pool: Money,
    /// Current status
    pub status: DrawStatus,
    /// Blockchain transaction hash for draw results
    pub blockchain_tx_hash: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl LotteryDraw {
    /// Create a new scheduled lottery draw
    pub fn new(draw_name: String, draw_date: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            draw_name,
            draw_date,
            winning_numbers: None,
            prize_pool: Money::zero(),
            status: DrawStatus::Scheduled,
            blockchain_tx_hash: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if draw is accepting tickets
    pub fn is_open(&self) -> bool {
        self.status == DrawStatus::Scheduled || self.status == DrawStatus::InProgress
    }

    /// Check if draw is completed
    pub fn is_completed(&self) -> bool {
        self.status == DrawStatus::Completed
    }

    /// Start accepting tickets
    pub fn open(&mut self) {
        if self.status == DrawStatus::Scheduled {
            self.status = DrawStatus::InProgress;
            self.updated_at = Utc::now();
        }
    }

    /// Add to prize pool (called when ticket is purchased)
    pub fn add_to_prize_pool(&mut self, amount: &Money) {
        self.prize_pool = Money::from_centimes(self.prize_pool.centimes() + amount.centimes());
        self.updated_at = Utc::now();
    }

    /// Complete the draw with winning numbers
    pub fn complete(&mut self, winning_numbers: Vec<i32>, tx_hash: String) {
        self.winning_numbers = Some(winning_numbers);
        self.blockchain_tx_hash = Some(tx_hash);
        self.status = DrawStatus::Completed;
        self.updated_at = Utc::now();
    }

    /// Cancel the draw
    pub fn cancel(&mut self) {
        self.status = DrawStatus::Cancelled;
        self.updated_at = Utc::now();
    }

    /// Check if a specific draw date has passed
    pub fn is_past_draw_date(&self) -> bool {
        Utc::now() > self.draw_date
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_new_draw() {
        let draw_date = Utc::now() + Duration::days(1);
        let draw = LotteryDraw::new("Test Draw".to_string(), draw_date);

        assert_eq!(draw.status, DrawStatus::Scheduled);
        assert!(draw.is_open());
        assert!(draw.winning_numbers.is_none());
    }

    #[test]
    fn test_complete_draw() {
        let mut draw = LotteryDraw::new("Test Draw".to_string(), Utc::now());
        draw.open();
        draw.complete(vec![1, 15, 23, 34, 42, 49], "tx_hash_123".to_string());

        assert!(draw.is_completed());
        assert_eq!(draw.winning_numbers, Some(vec![1, 15, 23, 34, 42, 49]));
    }
}
