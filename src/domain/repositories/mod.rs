//! Repository traits defining data access interfaces.
//!
//! These traits isolate the domain from infrastructure concerns,
//! following the Dependency Inversion principle.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{LotteryDraw, Ticket, TicketStatus, Transaction, User};
use crate::shared::errors::InfrastructureError;

/// Result type for repository operations
pub type RepoResult<T> = Result<T, InfrastructureError>;

/// User repository interface trait for managing user data.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Find user by ID
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<User>>;

    /// Find user by phone number
    async fn find_by_phone(&self, phone_number: &str) -> RepoResult<Option<User>>;

    /// Create a new user
    async fn create(&self, user: &User) -> RepoResult<()>;

    /// Update an existing user
    async fn update(&self, user: &User) -> RepoResult<()>;

    /// Check of phone number is already exists
    async fn phone_exists(&self, phone_number: &str) -> RepoResult<bool>;
}

/// Ticker repository interface trait for managing ticket data.
#[async_trait]
pub trait TicketRepository: Send + Sync {
    /// Find ticket by ID
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Ticket>>;

    /// Find all tickets for a user
    async fn find_by_user(&self, user_id: Uuid, limit: i64, offset: i64)
                          -> RepoResult<Vec<Ticket>>;

    /// Alias for find_by_user for convenience
    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> RepoResult<Vec<Ticket>> {
        self.find_by_user(user_id, limit, offset).await
    }

    /// Find all tickets for a lottery draw
    async fn find_by_draw(&self, draw_id: Uuid) -> RepoResult<Vec<Ticket>>;

    /// Find tickets by status
    async fn find_by_status(&self, status: TicketStatus) -> RepoResult<Vec<Ticket>>;

    /// Create a new ticket
    async fn create(&self, ticket: &Ticket) -> RepoResult<()>;

    /// Update existing ticket
    async fn update(&self, ticket: &Ticket) -> RepoResult<()>;

    /// Count tickets for a user
    async fn count_by_user(&self, user_id: Uuid) -> RepoResult<i64>;

}

/// Transaction repository interface trait for managing transaction data.
#[async_trait]
pub trait TransactionRepository: Send + Sync{
    /// Find transaction by ID
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Transaction>>;

    /// Find all transactions for a user
    async fn find_by_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> RepoResult<Vec<Transaction>>;

    /// Alias for find_by_user
    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> RepoResult<Vec<Transaction>> {
        self.find_by_user(user_id, limit, offset).await
    }

    /// Create a new transaction
    async fn create(&self, transaction: &Transaction) -> RepoResult<()>;

    /// Update existing transaction
    async fn update(&self, transaction: &Transaction) -> RepoResult<()>;

    /// Count transactions for a user
    async fn count_by_user(&self, user_id: Uuid) -> RepoResult<i64>;
}

/// Lottery draw repository interface trait for managing lottery draw data.
#[async_trait]
pub trait LotteryDrawRepository: Send + Sync {
    /// Find draw by ID
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<LotteryDraw>>;

    /// Find draw by name (e.g. "ROUND_123...")
    async fn find_by_name(&self, name: &str) -> RepoResult<Option<LotteryDraw>>;

    /// Find all draws with pagination
    async fn find_all(&self, limit: i64, offset: i64) -> RepoResult<Vec<LotteryDraw>>;

    /// Find open draws (available for ticket purchase)
    async fn find_open(&self) -> RepoResult<Vec<LotteryDraw>>;

    /// Find upcoming draws (scheduled or in progress)
    async fn find_upcoming(&self) -> RepoResult<Vec<LotteryDraw>>;

    /// Find completed draws
    async fn find_completed(&self, limit: i64) -> RepoResult<Vec<LotteryDraw>>;

    /// Create a new draw
    async fn create(&self, draw: &LotteryDraw) -> RepoResult<()>;

    /// Update existing draw
    async fn update(&self, draw: &LotteryDraw) -> RepoResult<()>;

}