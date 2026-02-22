//! Wave integration stub.
//!
//! API Documentation: https://developer.wave.com/
//! To be implemented when API credentials are obtained.

use async_trait::async_trait;

use super::{PaymentProvider, PaymentRequest, PaymentResult, PaymentStatus};
use crate::shared::errors::InfrastructureError;

/// Wave payment provider
pub struct WaveProvider {
    /// API base URL
    api_url: String,
    /// API key
    api_key: String,
    /// Webhook secret
    webhook_secret: String,
}

impl WaveProvider {
    pub fn new(api_url: &str, api_key: &str, webhook_secret: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            api_key: api_key.to_string(),
            webhook_secret: webhook_secret.to_string(),
        }
    }

    pub fn from_env() -> Option<Self> {
        let api_url = std::env::var("WAVE_API_URL").ok()?;
        let api_key = std::env::var("WAVE_API_KEY").ok()?;
        let webhook_secret = std::env::var("WAVE_WEBHOOK_SECRET").ok()?;

        Some(Self::new(&api_url, &api_key, &webhook_secret))
    }
}

#[async_trait]
impl PaymentProvider for WaveProvider {
    fn name(&self) -> &'static str {
        "Wave"
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn initiate_deposit(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, InfrastructureError> {
        // TODO: Implement Wave payment request API

        tracing::warn!(
            "Wave deposit initiated (STUB): {} FCFA to {}",
            request.amount.to_fcfa(),
            request.phone_number
        );

        Ok(PaymentResult {
            provider_ref: format!("WAVE-STUB-{}", request.transaction_id),
            status: PaymentStatus::Pending,
            message: Some("Stub: En attente de l'implémentation API Wave".to_string()),
        })
    }

    async fn initiate_withdrawal(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, InfrastructureError> {
        // TODO: Implement Wave payout API

        tracing::warn!(
            "Wave withdrawal initiated (STUB): {} FCFA to {}",
            request.amount.to_fcfa(),
            request.phone_number
        );

        Ok(PaymentResult {
            provider_ref: format!("WAVE-STUB-W-{}", request.transaction_id),
            status: PaymentStatus::Pending,
            message: Some("Stub: En attente de l'implémentation API Wave".to_string()),
        })
    }

    async fn check_status(&self, provider_ref: &str) -> Result<PaymentResult, InfrastructureError> {
        Ok(PaymentResult {
            provider_ref: provider_ref.to_string(),
            status: PaymentStatus::Pending,
            message: Some("Stub: Vérification de statut non implémentée".to_string()),
        })
    }

    fn verify_webhook_signature(&self, _payload: &[u8], _signature: &str) -> bool {
        false
    }
}
