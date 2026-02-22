//! Ticket entity representing a lottery ticket.

use std::fmt::Formatter;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::domain::value_objects::{Money,TicketNumbers};

/// Ticket status in the lottery system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ticket_status", rename_all = "snake_case")]
pub enum TicketStatus{
    /// Awaiting Blockchain Confirmation
    Pending,
    /// Confirmed on the blockchain
    Confirmed,
    /// Winning ticket
    Won,
    /// Losing ticket
    Lost,
    /// Ticket cancelled before draw
    Cancelled,
}

impl std::fmt::Display for TicketStatus{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            TicketStatus::Pending => write!(f, "pending"),
            TicketStatus::Confirmed => write!(f, "confirmed"),
            TicketStatus::Won => write!(f, "won"),
            TicketStatus::Lost => write!(f, "lost"),
            TicketStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Ticket entity representing a lottery ticket purchased by a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    /// Unique identifier
    pub id: Uuid,
    /// Owner user ID
    pub user_id: Uuid,
    /// Associated lottery draw ID
    pub lottery_draw_id: Uuid,
    /// Selected lottery numbers
    pub numbers: TicketNumbers,
    /// Price paid in centimes
    pub price: Money,
    /// Current status
    pub status: TicketStatus,
    /// Blockchain transaction hash for immutability proof
    pub blockchain_tx_hash: Option<String>,
    /// Prize won (0 if not won)
    pub prize_won: Money,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}