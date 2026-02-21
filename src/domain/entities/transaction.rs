// //! Transaction entity representing financial operations.
//
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;
//
// use crate::domain::value_objects::Money;
//
// /// Transaction Status
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
// #[sqlx(type_name = "transaction_status", rename_all = "snake_case")]
// pub enum TransactionStatus{
//     Pending,
//     Completed,
//     Failed,
//     Refunded,
// }
//
// impl std::fmt::Display for TransactionStatus{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TransactionStatus::Pending => write!(f, "pending"),
//             TransactionStatus::Completed => write!(f, "completed"),
//             TransactionStatus::Failed => write!(f, "failed"),
//             TransactionStatus::Refunded => write!(f, "refunded"),
//         }
//     }
// }
//
// /// Tansaction Type
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
// #[sqlx(type_name = "transaction_type", rename_all = "snake_case")]
// pub enum TransactionType {
//     /// Money deposited (Orange Money, MTN, etc.)
//     Deposit,
//     /// Money withdrawn
//     Withdrawal,
//     /// Ticket purchase
//     TicketPurchase,
//     /// Prize payout
//     PrizePayout,
//     /// Refund for cancelled ticket
//     Refund,
// }
//
// impl std::fmt::Display for TransactionType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TransactionType::Deposit => write!(f, "deposit"),
//             TransactionType::Withdrawal => write!(f, "withdrawal"),
//             TransactionType::TicketPurchase => write!(f, "ticket_purchase"),
//             TransactionType::PrizePayout => write!(f, "prize_payout"),
//             TransactionType::Refund => write!(f, "refund"),
//         }
//     }
// }
//
// /// Transaction entity representing a financial operation in the system.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Transaction {
//     /// Unique identifier
//     pub id: Uuid,
//     /// User who made the transaction
//     pub user_id: Uuid,
//     /// Amount in centimes (always positive)
//     pub amount: Money,
//     /// Type of transaction
//     pub transaction_type: TransactionType,
//     /// Current status
//     pub status: TransactionStatus,
//     /// External payment provider reference (Orange Money, MTN, etc.)
//     pub provider_ref: Option<String>,
//     /// Related ticket ID (for purchases/refunds)
//     pub ticket_id: Option<Uuid>,
//     /// Optional description
//     pub description: Option<String>,
//     /// Creation timestamp
//     pub created_at: DateTime<Utc>,
//     /// Last update timestamp
//     pub updated_at: DateTime<Utc>,
// }
//
//
// impl Transaction{
//     /// Create a new deposit transaction
//     pub fn deposit(user_id: Uuid, amount: Money, provider_ref: String) -> Self {
//         let now = Utc::now();
//         Self {
//             id: Uuid::new_v4(),
//             user_id,
//             amount,
//             transaction_type: TransactionType::Deposit,
//             status: TransactionStatus::Pending,
//             provider_ref: Some(provider_ref),
//             ticket_id: None,
//             description: Some("Dépôt de fonds".to_string()),
//             created_at: now,
//             updated_at: now,
//         }
//     }
//
//     /// Create a new ticket purchase transaction
//     pub fn ticket_purchase(user_id: Uuid, amount: Money, ticket_id: Uuid) -> Self {
//         let now = Utc::now();
//         Self {
//             id: Uuid::new_v4(),
//             user_id,
//             amount,
//             transaction_type: TransactionType::TicketPurchase,
//             status: TransactionStatus::Pending,
//             provider_ref: None,
//             ticket_id: Some(ticket_id),
//             description: Some("Achat de ticket".to_string()),
//             created_at: now,
//             updated_at: now,
//         }
//     }
//
//     /// Create a prize payout transaction
//     pub fn prize_payout(user_id: Uuid, amount: Money, ticket_id: Uuid) -> Self {
//         let now = Utc::now();
//         Self {
//             id: Uuid::new_v4(),
//             user_id,
//             amount,
//             transaction_type: TransactionType::PrizePayout,
//             status: TransactionStatus::Pending,
//             provider_ref: None,
//             ticket_id: Some(ticket_id),
//             description: Some("Paiement de gain".to_string()),
//             created_at: now,
//             updated_at: now,
//         }
//     }
//
//     /// Mark transaction as completed
//     pub fn complete(&mut self) {
//         self.status = TransactionStatus::Completed;
//         self.updated_at = Utc::now();
//     }
//
//     /// Mark transaction as failed
//     pub fn fail(&mut self) {
//         self.status = TransactionStatus::Failed;
//         self.updated_at = Utc::now();
//     }
//
//     /// Refund transaction
//     pub fn refund(&mut self) {
//         self.status = TransactionStatus::Refunded;
//         self.updated_at = Utc::now();
//     }
//
//     /// Check if transaction is completed
//     pub fn is_completed(&self) -> bool {
//         self.status == TransactionStatus::Completed
//     }
//
//     /// Check if transaction is a credit (adds money to user)
//     pub fn is_credit(&self) -> bool {
//         matches!(
//             self.transaction_type,
//             TransactionType::Deposit | TransactionType::PrizePayout | TransactionType::Refund
//         )
//     }
//
//     /// Check if transaction is a debit (removes money from user)
//     pub fn is_debit(&self) -> bool {
//         matches!(
//             self.transaction_type,
//             TransactionType::Withdrawal | TransactionType::TicketPurchase
//         )
//     }
//
// }