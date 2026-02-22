//! Payment provider trait and implementations.
//!
//! Stubs for Mobile Money integration (Orange, MTN, Wave).
//! To be implemented when API access is granted.

mod mtn_mobile_money;
mod orange_money;
mod wave;



use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::value_objects::Money;
use crate::shared::errors::InfrastructureError;

/// Payment status enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}


/// Payment transaction result
#[derive(Debug, Clone)]
pub struct PaymentResult {
    /// Provider's transaction reference
    pub provider_ref: String,
    /// Status of the payment
    pub status: PaymentStatus,
    /// Message from provider
    pub message: Option<String>,
}

/// Payment request for deposit or withdrawal
#[derive(Debug, Clone)]
pub struct PaymentRequest {
    /// Internal transaction ID
    pub transaction_id: Uuid,
    /// User's phone number (E.164 format)
    pub phone_number: String,
    /// Amount to pay
    pub amount: Money,
    /// Description shown to user
    pub description: String,
}




/// Trait for payment providers (Orange Money, MTN, Wave)
#[async_trait]
pub trait PaymentProvider: Send + Sync {
    /// Provider name (e.g., "Orange Money", "MTN Mobile Money", "Wave")
    fn name(&self) -> &'static str;

    /// Check if provider is available
    async fn is_available(&self) -> bool;

    /// Initiate a deposit (user pays to platform)
    async fn initiate_deposit(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, InfrastructureError>;

    /// Initiate a withdrawal (platform pays to user)
    async fn initiate_withdrawal(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, InfrastructureError>;

    /// Check payment status by provider reference
    async fn check_status(&self, provider_ref: &str) -> Result<PaymentResult, InfrastructureError>;

    /// Verify webhook signature (for callbacks)
    fn verify_webhook_signature(&self, payload: &[u8], signature: &str) -> bool;
}

/// Unified payment service that routes to appropriate provider
pub struct PaymentService {
    providers: Vec<Box<dyn PaymentProvider>>,
}

impl PaymentService {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Box<dyn PaymentProvider>) {
        self.providers.push(provider);
    }

    /// Get provider by phone number prefix (returns first available)
    pub fn get_provider_for_phone(&self, _phone: &str) -> Option<&dyn PaymentProvider> {
        // Ivory Coast prefixes:
        // Orange: +225 07, +225 08, +225 09
        // MTN: +225 05, +225 04
        // Wave: All numbers (wallet-based)

        // For now, return first provider
        // TODO: Match by phone prefix when implementing full logic
        self.providers.first().map(|p| p.as_ref())
    }
}
