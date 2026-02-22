//! MTN Mobile Money CI integration stub.
//!
//! API Documentation: https://momodeveloper.mtn.com/
//! To be implemented when API credentials are obtained.

use async_trait::async_trait;

use super::{PaymentProvider, PaymentRequest, PaymentResult, PaymentStatus};
use crate::shared::errors::{ApiError, InfrastructureError};


/// MTN Mobile Money Côte d'Ivoire provider
pub struct MtnMobileMoneyProvider {
    /// API base URL
    api_url: String,
    /// Subscription key
    subscription_key: String,
    /// API user ID
    api_user: String,
    /// API key
    api_key: String,
    /// Target environment (sandbox/production)
    environment: String,
}



impl MtnMobileMoneyProvider {
    pub fn new(
        api_url: &str,
        subscription_key: &str,
        api_user: &str,
        api_key: &str,
        environment: &str,
    ) -> Self {
        Self {
            api_url: api_url.to_string(),
            subscription_key: subscription_key.to_string(),
            api_user: api_user.to_string(),
            api_key: api_key.to_string(),
            environment: environment.to_string(),
        }
    }

    pub fn from_env() -> Option<Self> {
        let api_url = std::env::var("MTN_MOMO_API_URL").ok()?;
        let subscription_key = std::env::var("MTN_MOMO_SUBSCRIPTION_KEY").ok()?;
        let api_user = std::env::var("MTN_MOMO_API_USER").ok()?;
        let api_key = std::env::var("MTN_MOMO_API_KEY").ok()?;
        let environment =
            std::env::var("MTN_MOMO_ENVIRONMENT").unwrap_or_else(|_| "sandbox".to_string());

        Some(Self::new(
            &api_url,
            &subscription_key,
            &api_user,
            &api_key,
            &environment,
        ))
    }
}

#[async_trait]
impl PaymentProvider for MtnMobileMoneyProvider {
    fn name(&self) -> &'static str {
        "MTN Mobile Money CI"
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn initiate_deposit(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, InfrastructureError> {
        // TODO: Implement MTN Collection API
        // POST /collection/v1_0/requesttopay

        tracing::warn!(
            "MTN MoMo deposit initiated (STUB): {} FCFA to {}",
            request.amount.to_fcfa(),
            request.phone_number
        );

        Ok(PaymentResult {
            provider_ref: format!("MTN-STUB-{}", request.transaction_id),
            status: PaymentStatus::Pending,
            message: Some("Stub: En attente de l'implémentation API MTN MoMo".to_string()),
        })
    }

    async fn initiate_withdrawal(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResult, InfrastructureError> {
        // TODO: Implement MTN Disbursement API
        // POST /disbursement/v1_0/transfer

        tracing::warn!(
            "MTN MoMo withdrawal initiated (STUB): {} FCFA to {}",
            request.amount.to_fcfa(),
            request.phone_number
        );

        Ok(PaymentResult {
            provider_ref: format!("MTN-STUB-W-{}", request.transaction_id),
            status: PaymentStatus::Pending,
            message: Some("Stub: En attente de l'implémentation API MTN MoMo".to_string()),
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
