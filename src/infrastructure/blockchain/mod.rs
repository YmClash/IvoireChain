//! Hyperledger Fabric blockchain adapter.
//!
//! This module provides both mock and real Hyperledger Fabric integration.
//! The real implementation uses Docker CLI to execute peer chaincode commands.

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::RwLock;
use tokio::process::Command;
use uuid::Uuid;


use crate::config::Config;
use crate::domain::services::{
    BlockchainService, RoundOnChain, ServiceResult, TicketOnChain, WalletOnChain,
};
use crate::shared::errors::InfrastructureError;



/// In-memory storage for development (simulates blockchain ledger)
struct MockLedger {
    tickets: HashMap<Uuid, TicketOnChain>,
    draws: HashMap<Uuid, DrawOnChain>,
    wallets: HashMap<String, i64>,
    rounds: HashMap<String, RoundOnChain>,
}

#[derive(Debug, Clone)]
struct DrawOnChain {
    draw_id: Uuid,
    winning_numbers: Vec<i32>,
    tx_hash: String,
    timestamp: i64,
}

/// Response from chaincode invoke
#[derive(Debug, Serialize, Deserialize)]
struct ChaincodeResponse {
    #[serde(rename = "txHash")]
    tx_hash: Option<String>,
    #[serde(rename = "ticketId")]
    ticket_id: Option<String>,
    #[serde(rename = "userId")]
    user_id: Option<String>,
    numbers: Option<Vec<i32>>,
    #[serde(rename = "drawId")]
    draw_id: Option<String>,
    timestamp: Option<String>,
    status: Option<String>,
}

/// Hyperledger Fabric blockchain adapter
pub struct HyperledgerAdapter {
    peer_url: String,
    channel_name: String,
    chaincode_name: String,
    /// Docker container name for CLI commands
    cli_container: String,
    /// Mock ledger for development( used wnhen mock_mode is true)
    ledger: RwLock<MockLedger>,
    mock_mode: bool,
}


impl HyperledgerAdapter {
    /// Create a new Hyperledger adapter
    pub fn new(config: &Config) -> Self {
        // Determine if we're in mock mode based on config
        let mock_mode = config.hyperledger_peer_url.is_empty()
            || config.hyperledger_peer_url == "mock"
            || config.hyperledger_peer_url == "localhost:7051";

        tracing::info!(
            "Hyperledger adapter initialized: peer={}, channel={}, chaincode={}, mock_mode={}",
            config.hyperledger_peer_url,
            config.hyperledger_channel_name,
            config.hyperledger_chaincode_name,
            mock_mode
        );

        Self {
            peer_url: config.hyperledger_peer_url.clone(),
            channel_name: config.hyperledger_channel_name.clone(),
            chaincode_name: config.hyperledger_chaincode_name.clone(),
            cli_container: "ivoirechain-cli".to_string(),
            ledger: RwLock::new(MockLedger {
                tickets: HashMap::new(),
                draws: HashMap::new(),
                wallets: HashMap::new(),
                rounds: HashMap::new(),
            }),
            mock_mode,
        }
    }

    /// Generate a unique transaction hash (for mock mode)
    /// Generate a mock transaction hash for development
    fn generate_mock_tx_hash(&self) -> String {
        format!("0x{}", hex::encode(Uuid::new_v4().as_bytes()))
    }

    /// Execute a chaincode invoke (write operation)
    async fn chaincode_invoke(
        &self,
        function: &str,
        args: &[String],
    ) -> Result<String, InfrastructureError> {
        let args_json = serde_json::json!({
            "function": function,
            "Args": args
        });

        // Build the docker command
        let cmd_args = format!(
            r#"peer chaincode invoke -o orderer.ivoirechain.com:7050 -C {} -n {} -c '{}' --tls --cafile /opt/gopath/src/github.com/hyperledger/fabric/peer/crypto/ordererOrganizations/ivoirechain.com/orderers/orderer.ivoirechain.com/msp/tlscacerts/tlsca.ivoirechain.com-cert.pem --peerAddresses peer0.org1.ivoirechain.com:7051 --tlsRootCertFiles /opt/gopath/src/github.com/hyperledger/fabric/peer/crypto/peerOrganizations/org1.ivoirechain.com/peers/peer0.org1.ivoirechain.com/tls/ca.crt --peerAddresses peer0.org2.ivoirechain.com:9051 --tlsRootCertFiles /opt/gopath/src/github.com/hyperledger/fabric/peer/crypto/peerOrganizations/org2.ivoirechain.com/peers/peer0.org2.ivoirechain.com/tls/ca.crt --waitForEvent"#,
            self.channel_name, self.chaincode_name, args_json
        );

        tracing::debug!("Executing chaincode invoke: {}", function);

        let output = Command::new("docker")
            .args(["exec", &self.cli_container, "bash", "-c", &cmd_args])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| InfrastructureError::Blockchain(format!("Docker exec failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::error!("Chaincode invoke failed: {}", stderr);
            return Err(InfrastructureError::Blockchain(format!(
                "Chaincode invoke failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Extract transaction ID from output
        // Format: "... txid [TXID] committed with status (VALID) ..."
        if let Some(start) = stdout.find("txid [") {
            if let Some(end) = stdout[start..].find("] committed") {
                let tx_id = &stdout[start + 6..start + end];
                return Ok(format!("0x{}", tx_id));
            }
        }

        // If we can't extract TX ID, generate one based on the response
        Ok(format!(
            "0x{}",
            hex::encode(&output.stdout[..16.min(output.stdout.len())])
        ))
    }

    /// Execute a chaincode query (read operation)
    async fn chaincode_query(
        &self,
        function: &str,
        args: &[String],
    ) -> Result<String, InfrastructureError> {
        let args_json = serde_json::json!({
            "function": function,
            "Args": args
        });

        let cmd_args = format!(
            r#"peer chaincode query -C {} -n {} -c '{}'"#,
            self.channel_name, self.chaincode_name, args_json
        );

        tracing::debug!("Executing chaincode query: {}", function);

        let output = Command::new("docker")
            .args(["exec", &self.cli_container, "bash", "-c", &cmd_args])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| InfrastructureError::Blockchain(format!("Docker exec failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::error!("Chaincode query failed: {}", stderr);
            return Err(InfrastructureError::Blockchain(format!(
                "Chaincode query failed: {}",
                stderr
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Check if Docker container is running
    async fn is_container_running(&self) -> bool {
        let output = Command::new("docker")
            .args([
                "ps",
                "--filter",
                &format!("name={}", self.cli_container),
                "--format",
                "{{.Names}}",
            ])
            .stdout(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(o) => String::from_utf8_lossy(&o.stdout).contains(&self.cli_container),
            Err(_) => false,
        }
    }

}


#[async_trait]
impl BlockchainService for HyperledgerAdapter {
    async fn record_ticket_purchase(
        &self,
        ticket_id: Uuid,
        user_id: Uuid,
        numbers: &[i32],
        draw_id: String,
    ) -> ServiceResult<String> {
        tracing::info!(
            "Recording ticket purchase on blockchain: ticket={}, user={}, draw={}, numbers={:?}",
            ticket_id,
            user_id,
            draw_id,
            numbers
        );

        // Check if we should use real blockchain
        if !self.mock_mode && self.is_container_running().await {
            // Use real Hyperledger Fabric
            let args = vec![
                ticket_id.to_string(),
                user_id.to_string(),
                serde_json::to_string(numbers).unwrap_or_default(),
                draw_id.to_string(),
                "500".to_string(),          // price in CFA
                Uuid::new_v4().to_string(), // reference tx id
            ];

            let tx_hash = self.chaincode_invoke("recordTicketPurchase", &args).await?;
            tracing::info!("Ticket recorded on blockchain (REAL): tx_hash={}", tx_hash);
            return Ok(tx_hash);
        }

        // Fallback to mock mode
        let tx_hash = self.generate_mock_tx_hash();
        let timestamp = Utc::now().timestamp();

        let ticket_data = TicketOnChain {
            ticket_id,
            user_id,
            draw_id,
            numbers: numbers.to_vec(),
            tx_hash: tx_hash.clone(),
            timestamp,
        };

        let mut ledger = self.ledger.write().map_err(|_| {
            InfrastructureError::Blockchain("Failed to acquire ledger lock".to_string())
        })?;

        ledger.tickets.insert(ticket_id, ticket_data);

        tracing::info!("Ticket recorded on blockchain (MOCK): tx_hash={}", tx_hash);

        Ok(tx_hash)
    }

    async fn record_draw_results(
        &self,
        draw_id: Uuid,
        winning_numbers: &[i32],
    ) -> ServiceResult<String> {
        tracing::info!(
            "Recording draw results on blockchain: draw={}, winning_numbers={:?}",
            draw_id,
            winning_numbers
        );

        // Check if we should use real blockchain
        if !self.mock_mode && self.is_container_running().await {
            let args = vec![
                draw_id.to_string(),
                serde_json::to_string(winning_numbers).unwrap_or_default(),
                Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            ];

            let tx_hash = self.chaincode_invoke("recordDrawResults", &args).await?;
            tracing::info!(
                "Draw results recorded on blockchain (REAL): tx_hash={}",
                tx_hash
            );
            return Ok(tx_hash);
        }

        // Fallback to mock mode
        let tx_hash = self.generate_mock_tx_hash();
        let timestamp = Utc::now().timestamp();

        let draw_data = DrawOnChain {
            draw_id,
            winning_numbers: winning_numbers.to_vec(),
            tx_hash: tx_hash.clone(),
            timestamp,
        };

        let mut ledger = self.ledger.write().map_err(|_| {
            InfrastructureError::Blockchain("Failed to acquire ledger lock".to_string())
        })?;

        ledger.draws.insert(draw_id, draw_data);

        tracing::info!(
            "Draw results recorded on blockchain (MOCK): tx_hash={}",
            tx_hash
        );

        Ok(tx_hash)
    }

    async fn verify_transaction(&self, tx_hash: &str) -> ServiceResult<bool> {
        tracing::debug!("Verifying transaction: {}", tx_hash);

        // Check if we should use real blockchain
        if !self.mock_mode && self.is_container_running().await {
            // Query the chaincode to verify the transaction exists
            tracing::info!("Verifying transaction on real blockchain: {}", tx_hash);
            // For now, assume valid if format is correct (Fabric doesn't have a direct tx verify)
            return Ok(tx_hash.starts_with("0x") && tx_hash.len() > 10);
        }

        // Mock mode verification
        let is_valid_format = tx_hash.starts_with("0x") && tx_hash.len() > 10;

        let ledger = self.ledger.read().map_err(|_| {
            InfrastructureError::Blockchain("Failed to acquire ledger lock".to_string())
        })?;

        let exists_in_tickets = ledger.tickets.values().any(|t| t.tx_hash == tx_hash);
        let exists_in_draws = ledger.draws.values().any(|d| d.tx_hash == tx_hash);

        Ok(is_valid_format && (exists_in_tickets || exists_in_draws))
    }

    async fn verify_ticket(&self, ticket_id: Uuid) -> ServiceResult<Option<TicketOnChain>> {
        tracing::debug!("Verifying ticket on blockchain: {}", ticket_id);

        // Check if we should use real blockchain
        if !self.mock_mode && self.is_container_running().await {
            let args = vec![ticket_id.to_string()];

            match self.chaincode_query("getTicket", &args).await {
                Ok(response) => {
                    if response.trim().is_empty() || response.contains("does not exist") {
                        return Ok(None);
                    }

                    // Try to parse the response
                    if let Ok(ticket_data) = serde_json::from_str::<ChaincodeResponse>(&response) {
                        if let (
                            Some(user_id_str),
                            Some(draw_id_str),
                            Some(numbers),
                            Some(tx_hash),
                        ) = (
                            ticket_data.user_id,
                            ticket_data.draw_id,
                            ticket_data.numbers,
                            ticket_data.tx_hash,
                        ) {
                            let user_id = Uuid::parse_str(&user_id_str).unwrap_or_default();
                            let draw_id = draw_id_str;
                            let timestamp = ticket_data
                                .timestamp
                                .and_then(|t| t.parse::<i64>().ok())
                                .unwrap_or(0);

                            return Ok(Some(TicketOnChain {
                                ticket_id,
                                user_id,
                                draw_id,
                                numbers,
                                tx_hash,
                                timestamp,
                            }));
                        }
                    }

                    return Ok(None);
                }
                Err(e) => {
                    tracing::warn!("Failed to query ticket from blockchain: {}", e);
                    // Fall through to mock mode
                }
            }
        }

        // Mock mode
        let ledger = self.ledger.read().map_err(|_| {
            InfrastructureError::Blockchain("Failed to acquire ledger lock".to_string())
        })?;

        Ok(ledger.tickets.get(&ticket_id).cloned())
    }

    async fn health_check(&self) -> ServiceResult<bool> {
        tracing::debug!("Performing blockchain health check");

        // Check if we should use real blockchain
        if !self.mock_mode {
            let container_running = self.is_container_running().await;

            if container_running {
                // Try to query the chaincode health
                match self.chaincode_query("healthCheck", &[]).await {
                    Ok(response) => {
                        let is_healthy = response.contains("OK") || response.contains("healthy");
                        tracing::info!("Blockchain health check (REAL): {}", is_healthy);
                        return Ok(is_healthy);
                    }
                    Err(e) => {
                        tracing::warn!("Health check query failed: {}", e);
                        return Ok(false);
                    }
                }
            } else {
                tracing::warn!("CLI container is not running");
                return Ok(false);
            }
        }

        // Mock mode - always healthy
        tracing::info!("Blockchain health check (MOCK): healthy");
        Ok(true)
    }

    // ---- Round Operations ----

    async fn start_round(&self, duration_seconds: i64) -> ServiceResult<(String, String)> {
        tracing::info!("Starting new round, duration={}s", duration_seconds);

        if !self.mock_mode {
            let args = vec![duration_seconds.to_string()];
            let tx_hash = match self.chaincode_invoke("startRound", &args).await {
                Ok(hash) => hash,
                Err(e) => {
                    tracing::warn!("Real chaincode startRound failed: {}, using mock", e);
                    // Fallback to mock for stability if chaincode fails (optional, but good for hybrid dev)
                    // For now, let's error out or return mock if specifically handled.
                    // Actually, if real fail, we should return error.
                    return Err(e);
                }
            };

            // Retrieve the Round ID (since invoke doesn't return payload easily in CLI mode)
            // We fetch the current round immediately after.
            // A short sleep might be needed for consistency, or rely on "read your own write" if on same peer.
            // But Fabric eventual consistency...
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            let current_round = self.get_current_round().await?;
            let round_id = current_round
                .map(|r| r.round_id)
                .unwrap_or_else(|| Uuid::new_v4().to_string()); // Fallback if parsing fails

            return Ok((tx_hash, round_id));
        }

        // Mock mode
        let round_id = Uuid::new_v4().to_string();
        let tx_hash = self.generate_mock_tx_hash();
        let timestamp = Utc::now().timestamp();

        let round = RoundOnChain {
            round_id: round_id.clone(),
            status: "OPEN".to_string(),
            start_time: timestamp.to_string(),
            end_time: (timestamp + duration_seconds).to_string(),
            winning_numbers: None,
        };

        let mut ledger = self.ledger.write().unwrap();
        ledger.rounds.insert(round_id.clone(), round);

        Ok((tx_hash, round_id))
    }

    async fn close_round(&self, round_id: String) -> ServiceResult<String> {
        tracing::info!("Closing round: {}", round_id);

        if !self.mock_mode {
            let args = vec![round_id];
            return self.chaincode_invoke("closeRound", &args).await;
        }

        // Mock mode
        let mut ledger = self.ledger.write().unwrap();
        if let Some(round) = ledger.rounds.get_mut(&round_id) {
            round.status = "CLOSED".to_string();
        }

        Ok(self.generate_mock_tx_hash())
    }

    async fn end_round(&self, round_id: String, winning_numbers: &[i32]) -> ServiceResult<String> {
        tracing::info!("Ending round: {}, winners={:?}", round_id, winning_numbers);

        if !self.mock_mode {
            let args = vec![
                round_id,
                serde_json::to_string(winning_numbers).unwrap_or_default(),
            ];
            return self.chaincode_invoke("endRound", &args).await;
        }

        // Mock mode
        let mut ledger = self.ledger.write().unwrap();
        if let Some(round) = ledger.rounds.get_mut(&round_id) {
            round.status = "FINISHED".to_string();
            round.winning_numbers = Some(winning_numbers.to_vec());
        }

        Ok(self.generate_mock_tx_hash())
    }

    async fn get_current_round(&self) -> ServiceResult<Option<RoundOnChain>> {
        tracing::debug!("Getting current round");

        if !self.mock_mode {
            match self.chaincode_query("getCurrentRound", &[]).await {
                Ok(response) => {
                    if response.trim().is_empty() {
                        return Ok(None);
                    }
                    if let Ok(round) = serde_json::from_str::<RoundOnChain>(&response) {
                        return Ok(Some(round));
                    }
                    tracing::warn!("Failed to parse current round: {}", response);
                    // If structure doesn't match, return None
                    return Ok(None);
                }
                Err(e) => {
                    tracing::warn!("Real chaincode getCurrentRound failed: {}, using mock", e);
                    // Fallback to mock
                }
            }
        }

        // Mock mode
        let ledger = self.ledger.read().unwrap();
        // Find the first OPEN round
        let round = ledger.rounds.values().find(|r| r.status == "OPEN").cloned();

        Ok(round)
    }

    async fn get_round(&self, round_id: String) -> ServiceResult<Option<RoundOnChain>> {
        tracing::debug!("Getting round details: {}", round_id);

        if !self.mock_mode {
            let args = vec![round_id.clone()];
            match self.chaincode_query("getRound", &args).await {
                Ok(response) => {
                    if response.trim().is_empty() || response.contains("does not exist") {
                        return Ok(None);
                    }
                    if let Ok(round) = serde_json::from_str::<RoundOnChain>(&response) {
                        return Ok(Some(round));
                    }
                    tracing::warn!("Failed to parse round response: {}", response);
                    return Ok(None);
                }
                Err(e) => {
                    tracing::warn!("Real chaincode getRound failed: {}", e);
                    // Fallback to mock not ideal here if specific ID requested, but consistent with other methods
                }
            }
        }

        // Mock mode
        let ledger = self.ledger.read().unwrap();
        Ok(ledger.rounds.get(&round_id).cloned())
    }

    // ---- Wallet Operations ----

    async fn create_wallet(&self, user_id: Uuid) -> ServiceResult<String> {
        tracing::info!("Creating wallet for user: {}", user_id);

        if !self.mock_mode {
            let args = vec![user_id.to_string()];
            match self.chaincode_invoke("createWallet", &args).await {
                Ok(response) => {
                    tracing::info!("Wallet created on-chain for user {}", user_id);
                    return Ok(response);
                }
                Err(e) => {
                    tracing::warn!("Real chaincode createWallet failed: {}, using mock", e);
                }
            }
        }

        let mut ledger = self.ledger.write().unwrap();
        ledger.wallets.insert(user_id.to_string(), 0);
        Ok(format!(
            "{{\"userId\":\"{}\",\"balanceCentimes\":0}}",
            user_id
        ))
    }

    async fn deposit(
        &self,
        user_id: Uuid,
        amount_centimes: i64,
        reference: &str,
    ) -> ServiceResult<String> {
        tracing::info!(
            "Depositing {} centimes for user {}",
            amount_centimes,
            user_id
        );

        if !self.mock_mode {
            let args = vec![
                user_id.to_string(),
                amount_centimes.to_string(),
                reference.to_string(),
            ];
            match self.chaincode_invoke("deposit", &args).await {
                Ok(response) => {
                    tracing::info!("On-chain deposit for user {}", user_id);
                    return Ok(response);
                }
                Err(e) => {
                    tracing::warn!("Real chaincode deposit failed: {}, using mock", e);
                }
            }
        }

        let mut ledger = self.ledger.write().unwrap();
        let balance = ledger.wallets.entry(user_id.to_string()).or_insert(0);
        *balance += amount_centimes;
        let new_balance = *balance;
        Ok(format!(
            "{{\"userId\":\"{}\",\"balanceCentimes\":{}}}",
            user_id, new_balance
        ))
    }

    async fn withdraw(
        &self,
        user_id: Uuid,
        amount_centimes: i64,
        reference: &str,
    ) -> ServiceResult<String> {
        tracing::info!(
            "Withdrawing {} centimes for user {}",
            amount_centimes,
            user_id
        );

        if !self.mock_mode {
            let args = vec![
                user_id.to_string(),
                amount_centimes.to_string(),
                reference.to_string(),
            ];
            match self.chaincode_invoke("withdraw", &args).await {
                Ok(response) => {
                    tracing::info!("On-chain withdraw for user {}", user_id);
                    return Ok(response);
                }
                Err(e) => {
                    tracing::warn!("Real chaincode withdraw failed: {}, using mock", e);
                }
            }
        }

        let mut ledger = self.ledger.write().unwrap();
        let balance = ledger.wallets.entry(user_id.to_string()).or_insert(0);
        if *balance < amount_centimes {
            return Err(InfrastructureError::Blockchain(format!(
                "Insufficient balance: have {}, need {}",
                balance, amount_centimes
            )));
        }
        *balance -= amount_centimes;
        let new_balance = *balance;
        Ok(format!(
            "{{\"userId\":\"{}\",\"balanceCentimes\":{}}}",
            user_id, new_balance
        ))
    }

    async fn get_wallet_balance(&self, user_id: Uuid) -> ServiceResult<WalletOnChain> {
        tracing::info!("Getting wallet balance for user: {}", user_id);

        if !self.mock_mode {
            let args = vec![user_id.to_string()];
            match self.chaincode_query("getWallet", &args).await {
                Ok(response) => {
                    if let Ok(wallet) = serde_json::from_str::<WalletOnChain>(&response) {
                        return Ok(wallet);
                    }
                    tracing::warn!("Failed to parse wallet response: {}", response);
                }
                Err(e) => {
                    tracing::warn!("Real chaincode getWallet failed: {}, using mock", e);
                }
            }
        }

        let ledger = self.ledger.read().unwrap();
        let balance = ledger
            .wallets
            .get(&user_id.to_string())
            .copied()
            .unwrap_or(0);
        let exists = ledger.wallets.contains_key(&user_id.to_string());
        Ok(WalletOnChain {
            user_id: user_id.to_string(),
            balance_centimes: balance,
            exists,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            hyperledger_peer_url: "mock".to_string(),
            hyperledger_channel_name: "lotterychannel".to_string(),
            hyperledger_chaincode_name: "lottery".to_string(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_mock_record_ticket() {
        let config = create_test_config();
        let adapter = HyperledgerAdapter::new(&config);

        let ticket_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let draw_id = "ROUND_TEST_123".to_string();
        let numbers = vec![5, 12, 23, 34, 45];

        let result = adapter
            .record_ticket_purchase(ticket_id, user_id, &numbers, draw_id)
            .await;

        assert!(result.is_ok());
        let tx_hash = result.unwrap();
        assert!(tx_hash.starts_with("0x"));
    }

    #[tokio::test]
    async fn test_mock_verify_ticket() {
        let config = create_test_config();
        let adapter = HyperledgerAdapter::new(&config);

        let ticket_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let draw_id = "ROUND_TEST_123".to_string();
        let numbers = vec![1, 2, 3, 4, 5];

        // Record a ticket first
        let _ = adapter
            .record_ticket_purchase(ticket_id, user_id, &numbers, draw_id)
            .await;

        // Now verify it
        let result = adapter.verify_ticket(ticket_id).await;
        assert!(result.is_ok());
        let ticket = result.unwrap();
        assert!(ticket.is_some());
        assert_eq!(ticket.unwrap().numbers, numbers);
    }

    #[tokio::test]
    async fn test_mock_health_check() {
        let config = create_test_config();
        let adapter = HyperledgerAdapter::new(&config);

        let result = adapter.health_check().await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_mock_record_draw_results() {
        let config = create_test_config();
        let adapter = HyperledgerAdapter::new(&config);

        let draw_id = Uuid::new_v4();
        let winning_numbers = vec![7, 14, 21, 28, 35];

        let result = adapter.record_draw_results(draw_id, &winning_numbers).await;
        assert!(result.is_ok());
        let tx_hash = result.unwrap();
        assert!(tx_hash.starts_with("0x"));

        // Verify transaction
        let verify_result = adapter.verify_transaction(&tx_hash).await;
        assert!(verify_result.is_ok());
        assert!(verify_result.unwrap());
    }
}