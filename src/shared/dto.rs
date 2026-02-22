use chrono::{DateTime, Utc};
use phonenumber::PhoneNumber;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;


// Authentication DTOs

/// Login request DTO
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 8, max = 20))]
    pub phone_number: String,  // Arrevoir si on vas paas mettre un i32 a un string
    #[validate(length(min = 6))]
    pub password: String,
}

/// Registration request DTO
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 8, max = 20))]
    pub phone_number: String,
    #[validate(length(
        min = 6,
        message = "Le mot de passe doit contenir au moins 6 caractères"
    ))]
    pub password: String,
}

/// Authentication response with JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}


/// User DTOs


/// user profile response DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id : Uuid,
    pub phone_number: PhoneNumber,
    pub balance_fcfa: f64,
    pub status: String,
    pub created_at : DateTime<Utc>,
}


/// Balance response
#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub balance_fcfa: f64,
    pub pending_fcfa: f64,
}


/// Ticket DTOs


/// Buy ticket request DTO
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BuyTicketResquest {
    pub lottery_draw_id: Uuid,
    #[validate(length(min = 1, max = 6, message = "Sélectionnez  1 et 6 numéros"))]
    pub numbers : Vec<i32>,
}


/// Ticket response
#[derive(Debug, Serialize, Deserialize)]
pub struct TicketResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub lottery_draw_id: Uuid,
    pub numbers: Vec<i32>,
    pub price_fcfa: f64,
    pub status: String,
    pub blockchain_tx_hash: Option<String>,
    pub prize_won_fcfa: f64,
    pub created_at : DateTime<Utc>,
}


/// Lottery Draw DTOs

/// Lottery draw response
#[derive(Debug, Serialize, Deserialize)]
pub struct LotteryDrawResponse {
    pub id: Uuid,
    pub draw_name: String,
    pub draw_date: DateTime<Utc>,
    pub winning_numbers: Option<Vec<i32>>,
    pub prize_pool_fcfa: f64,
    pub status: String,
    pub created_at : DateTime<Utc>,
}

/// Transaction DTOs

/// Transaction response
#[derive(Debug, Serialize, Deserialize,Validate)]
pub struct DepositRequest {
    /// Amount in FCFA
    #[validate(range(min = 100, message = "Le montant minimum est de 100 FCFA"))]
    pub amount_fcfa: f64,
    /// Payment provider reference
    pub provider_ref: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount_fcfa: f64,
    pub transaction_type: String, // "deposit" or "withdrawal"
    pub status: String, // "pending", "completed", "failed"
    pub provider_ref: Option<String>,
    pub blockchain_tx_hash: Option<String>, //a voir
    pub created_at : DateTime<Utc>,
}


/// Health check response

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}


/// Pagination DTOs

/// Pagination request parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams{
    pub page: Option<u32>,
    pub limit : Option<i64>,
    pub offset: Option<i64>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total: i64,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u32, limit: u32, total: i64) -> Self {
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        Self {
            data,
            page,
            limit,
            total,
            total_pages,
        }
    }
}

/// Additional DTOs can be defined here as needed for other features like monitoring, admin operations, etc.


/// Buy ticket response
#[derive(Debug, Serialize)]
pub struct BuyTicketResponse {
    pub ticket_id: Uuid,
    pub numbers: Vec<i32>,
    pub draw_id: Uuid,
    pub price_fcfa: f64,
    pub status: String,
    pub message: String,
}

/// Update profile request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 8, max = 20))]
    pub phone_number: Option<String>,
}

/// User profile response (extended)
#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub phone_number: String,
    pub balance_fcfa: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Draw response (alias for handlers)
pub type DrawResponse = LotteryDrawResponse;

/// Draw result response
#[derive(Debug, Serialize)]
pub struct DrawResultResponse {
    pub draw_id: Uuid,
    pub draw_name: String,
    pub draw_date: DateTime<Utc>,
    pub winning_numbers: Vec<i32>,
    pub total_tickets: i64,
    pub total_winners: i64,
    pub prize_pool_fcfa: f64,
}
