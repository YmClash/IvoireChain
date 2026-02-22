//! Orange Money CI integration stub.
//!
//! API Documentation: https://developer.orange.com/apis/om-ci
//! To be implemented when API credentials are obtained.

use async_trait::async_trait;

use super::{PaymentProvider, PaymentRequest, PaymentResult, PaymentStatus};
use crate::shared::errors::InfrastructureError;

/// Orange Money Côte d'Ivoire provider
pub struct OrangeMoneyProvider {
    /// API base URL
    api_url: String,
    /// Merchant ID
    merchant_id: String,
    /// API key
    api_key: String,
    /// Webhook secret for signature verification
    webhook_secret: String,
}

impl OrangeMoneyProvider {
    pub fn new(api_url: &str, merchant_id: &str, api_key: &str, webhook_secret: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            merchant_id: merchant_id.to_string(),
            api_key: api_key.to_string(),
            webhook_secret: webhook_secret.to_string(),
        }
    }

    /// Create from environment variables
    pub fn from_env() -> Option<Self> {
        let api_url = std::env::var("ORANGE_MONEY_API_URL").ok()?;
        let merchant_id = std::env::var("ORANGE_MONEY_MERCHANT_ID").ok()?;
        let api_key = std::env::var("ORANGE_MONEY_API_KEY").ok()?;
        let webhook_secret = std::env::var("ORANGE_MONEY_WEBHOOK_SECRET").ok()?;

        Some(Self::new(&api_url, &merchant_id, &api_key, &webhook_secret))
    }
}

#[async_trait]
impl PaymentProvider for OrangeMoneyProvider {
    fn name(&self) -> &'static str {
        "Orange Money CI"
    }

    async fn is_available(&self) -> bool {
        // TODO: Ping API health endpoint
        !self.api_key.is_empty()
    }

    async fn initiate_deposit(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, InfrastructureError> {
        // TODO: Implement actual API call
        // POST /orange-money-webpay/ci/v1/webpay
        // {
        //   "merchant_key": self.api_key,
        //   "currency": "OUV",
        //   "order_id": request.transaction_id,
        //   "amount": request.amount.to_fcfa(),
        //   "return_url": "...",
        //   "cancel_url": "...",
        //   "notif_url": "...",
        //   "lang": "fr"
        // }

        tracing::warn!(
            "Orange Money deposit initiated (STUB): {} FCFA to {}",
            request.amount.to_fcfa(),
            request.phone_number
        );

        Ok(PaymentResult {
            provider_ref: format!("OM-STUB-{}", request.transaction_id),
            status: PaymentStatus::Pending,
            message: Some("Stub: En attente de l'implémentation API Orange Money".to_string()),
        })
    }

    async fn initiate_withdrawal(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, InfrastructureError> {
        // TODO: Implement actual disbursement API call

        tracing::warn!(
            "Orange Money withdrawal initiated (STUB): {} FCFA to {}",
            request.amount.to_fcfa(),
            request.phone_number
        );

        Ok(PaymentResult {
            provider_ref: format!("OM-STUB-W-{}", request.transaction_id),
            status: PaymentStatus::Pending,
            message: Some("Stub: En attente de l'implémentation API Orange Money".to_string()),
        })
    }

    async fn check_status(&self, provider_ref: &str) -> Result<PaymentResult, InfrastructureError> {
        // TODO: Call status API

        Ok(PaymentResult {
            provider_ref: provider_ref.to_string(),
            status: PaymentStatus::Pending,
            message: Some("Stub: Vérification de statut non implémentée".to_string()),
        })
    }

    fn verify_webhook_signature(&self, _payload: &[u8], _signature: &str) -> bool {
        // TODO: Verify HMAC signature with webhook_secret
        false
    }
}
