//! Configuration module for loading and validating environment variables.

use std::env;



/// Application configuration loaded from environment variables.
#[derive(Debug,Clone,Default)]
pub struct Config {
    // Server
    pub server_host: String,
    pub server_port: u16,

    // Database
    pub database_url: String,
    pub database_max_connections: u32,

    // Redis
    pub redis_url: String,

    // JWT
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,

    // Argon2
    pub argon2_memory_cost: u32,
    pub argon2_time_cost: u32,
    pub argon2_parallelism: u32,

    // Hyperledger
    pub hyperledger_peer_url: String,
    pub hyperledger_channel_name: String,
    pub hyperledger_chaincode_name: String,

    // Application
    pub ticket_price_fcfa: i64,
    pub max_numbers_per_ticket: usize,
    pub number_range_min: i32,
    pub number_range_max: i32,

}

impl Config{
    /// Load configuration from environment variables.
    ///
    /// # Panics
    ///
    /// Panics if required environment variables are missing.
    pub fn from_env() -> Self {
        Self {
            // Server
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid port number"),

            // Database
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DATABASE_MAX_CONNECTIONS must be a number"),

            // Redis
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),

            // JWT
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .expect("JWT_EXPIRATION_HOURS must be a number"),

            // Argon2
            argon2_memory_cost: env::var("ARGON2_MEMORY_COST")
                .unwrap_or_else(|_| "65536".to_string())
                .parse()
                .expect("ARGON2_MEMORY_COST must be a number"),
            argon2_time_cost: env::var("ARGON2_TIME_COST")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .expect("ARGON2_TIME_COST must be a number"),
            argon2_parallelism: env::var("ARGON2_PARALLELISM")
                .unwrap_or_else(|_| "4".to_string())
                .parse()
                .expect("ARGON2_PARALLELISM must be a number"),

            // Hyperledger
            hyperledger_peer_url: env::var("HYPERLEDGER_PEER_URL")
                .unwrap_or_else(|_| "grpc://localhost:7051".to_string()),
            hyperledger_channel_name: env::var("HYPERLEDGER_CHANNEL_NAME")
                .unwrap_or_else(|_| "lottery-channel".to_string()),
            hyperledger_chaincode_name: env::var("HYPERLEDGER_CHAINCODE_NAME")
                .unwrap_or_else(|_| "lottery-contract".to_string()),

            // Application
            ticket_price_fcfa: env::var("TICKET_PRICE_FCFA")
                .unwrap_or_else(|_| "500".to_string())
                .parse()
                .expect("TICKET_PRICE_FCFA must be a number"),
            max_numbers_per_ticket: env::var("MAX_NUMBERS_PER_TICKET")
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .expect("MAX_NUMBERS_PER_TICKET must be a number"),
            number_range_min: env::var("NUMBER_RANGE_MIN")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .expect("NUMBER_RANGE_MIN must be a number"),
            number_range_max: env::var("NUMBER_RANGE_MAX")
                .unwrap_or_else(|_| "49".to_string())
                .parse()
                .expect("NUMBER_RANGE_MAX must be a number"),
        }

    }

    /// Get the server bind address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    /// Get Ticket in centimes
    pub fn ticket_price_centimes(&self) -> i64 {
        self.ticket_price_fcfa * 100
    }

}