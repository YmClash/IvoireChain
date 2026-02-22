//! Service traits for external integrations.
//!
//! These traits define interfaces for services that interact
//! with external systems (blockchain, cache, payments).

use async_trait::async_trait;
use uuid::Uuid;

use crate::shared::errors::InfrastructureError;

/// Result type for service operations
pub type ServiceResult<T> = Result<T, InfrastructureError>;

/// Blockchain service interface trait for interacting with the blockchain.
#[async_trait]
pub trait BlockchainService: Send + Sync {
    /// Record a ticket purchase on the blockchain
    ///
    /// Returns the transaction hash
    async fn record_ticket_purchase(
        &self,
        ticket_id: Uuid,
        user_id: Uuid,
        numbers: &[i32],
        draw_id: String,
    ) -> ServiceResult<String>;

    /// Record lottery draw results on the blockchain
    ///
    /// Returns the transaction hash
    async fn record_draw_results(
        &self,
        draw_id: Uuid,
        winning_numbers: &[i32],
    ) -> ServiceResult<String>;

    /// Verify a transaction exists on the blockchain
    async fn verify_transaction(&self, tx_hash: &str) -> ServiceResult<bool>;

    /// Verify a ticket exists and get its recorded data
    async fn verify_ticket(&self, ticket_id: Uuid) -> ServiceResult<Option<TicketOnChain>>;

    /// Check if blockchain connection is healthy
    async fn health_check(&self) -> ServiceResult<bool>;

    // ---- Wallet Operations ----

    /// Create a wallet for a user on the blockchain
    async fn create_wallet(&self, user_id: Uuid) -> ServiceResult<String>;

    /// Deposit funds into a user's on-chain wallet
    async fn deposit(
        &self,
        user_id: Uuid,
        amount_centimes: i64,
        reference: &str,
    ) -> ServiceResult<String>;

    /// Withdraw funds from a user's on-chain wallet
    async fn withdraw(
        &self,
        user_id: Uuid,
        amount_centimes: i64,
        reference: &str,
    ) -> ServiceResult<String>;

    /// Get the on-chain wallet balance for a user
    async fn get_wallet_balance(&self, user_id: Uuid) -> ServiceResult<WalletOnChain>;

    // ---- Round Operations ----

    /// Start a new lottery round
    ///
    /// Returns the transaction hash and the new round ID
    /// Start a new lottery round
    ///
    /// Returns the transaction hash and the new round ID
    async fn start_round(&self, duration_seconds: i64) -> ServiceResult<(String, String)>;

    /// Close the current round (no more tickets)
    ///
    /// Returns the transaction hash
    async fn close_round(&self, round_id: String) -> ServiceResult<String>;

    /// End the round and declare winners
    ///
    /// Returns the transaction hash
    async fn end_round(&self, round_id: String, winning_numbers: &[i32]) -> ServiceResult<String>;

    /// Get the current active round
    async fn get_current_round(&self) -> ServiceResult<Option<RoundOnChain>>;

    /// Get a specific round by ID
    async fn get_round(&self, round_id: String) -> ServiceResult<Option<RoundOnChain>>;

}

/// Ticket data as recorded on the blockchain
#[derive(Debug, Clone)]
pub struct TicketOnChain {
    pub ticket_id: Uuid,
    pub user_id: Uuid,
    pub draw_id: String,
    pub numbers: Vec<i32>,
    pub tx_hash: String,
    pub timestamp: i64,
}


/// Wallet data as recorded on the blockchain
#[derive(Debug, Clone,serde::Serialize, serde::Deserialize)]
pub struct WalletOnChain {
    #[serde(alias = "userId")]
    pub user_id: String,
    #[serde(alias = "balanceCentimes")]
    pub balance_centimes: i64,
    pub exists: bool,
}

/// Round data as recorded on the blockchain
#[derive(Debug, Clone,serde::Serialize, serde::Deserialize)]
pub struct RoundOnChain {
    #[serde(alias = "roundId")]
    pub round_id: String,
    pub status: String,
    #[serde(alias = "startTime")]
    pub start_time: String,
    #[serde(alias = "endTime")]
    pub end_time: String,
    #[serde(alias = "winningNumbers")]
    pub winning_numbers: Option<Vec<i32>>,
}

/// Cache service interface trait for caching frequently accessed data.
#[async_trait]
pub trait CacheService: Send + Sync {
    /// Get a value from cache
    async fn get(&self, key: &str) -> ServiceResult<Option<String>>;

    /// Set a value in cache with expiration (in seconds)
    async fn set(&self, key: &str, value: &str, expire_seconds: u64) -> ServiceResult<()>;

    /// Delete a value from cache
    async fn delete(&self, key: &str) -> ServiceResult<()>;

    /// Check if key exists
    async fn exists(&self, key: &str) -> ServiceResult<bool>;

    /// Set hash field
    async fn hset(&self, key: &str, field: &str, value: &str) -> ServiceResult<()>;

    /// Get hash field
    async fn hget(&self, key: &str, field: &str) -> ServiceResult<Option<String>>;

    /// Increment a counter
    async fn incr(&self, key: &str) -> ServiceResult<i64>;

}


/// Password hashing sertice interface trait for secure password management.
#[async_trait]
pub trait PasswordHashingService: Send + Sync{
    /// Hash a password
    fn hash_password(&self, password: &str) -> ServiceResult<String>;

    /// Verify a password against a hash
    fn verify_password(&self, password: &str, hashed: &str) -> ServiceResult<bool>;
}

/// JWT token service interface trait for generating and validating JWTs.
#[async_trait]
pub trait TokenService: Send + Sync{
    /// Generate a JWT token for a user
    fn generate_token(&self, user_id: Uuid) -> ServiceResult<String>;

    /// Validate a JWT token and extract the user ID
    fn validate_token(&self, token: &str) -> ServiceResult<Uuid>;
}