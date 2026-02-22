//! Redis cache service implementation.

use async_trait::async_trait;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;

use crate::config::Config;
use crate::domain::services::{CacheService, ServiceResult};
use crate::shared::errors::InfrastructureError;

/// Redis cache implementation
pub struct RedisCache {
    conn: ConnectionManager,
}

impl RedisCache {
    /// Create a new Redis cache connection
    pub async fn new(config: &Config) -> Result<Self, InfrastructureError> {
        let client = redis::Client::open(config.redis_url.as_str())
            .map_err(|e| InfrastructureError::Cache(format!("Redis client error: {}", e)))?;

        let conn = ConnectionManager::new(client)
            .await
            .map_err(|e| InfrastructureError::Cache(format!("Redis connection error: {}", e)))?;

        tracing::info!("Redis connection established");

        Ok(Self { conn })
    }
}
//
// #[async_trait]
// impl CacheService for RedisCache {
//     // async fn get(&self, key: &str) -> ServiceResult<Option<String>> {
//     //     let mut conn = self.conn.clone();
//     //     let value: Option<String> = conn
//     //         .get(key)
//     //         .await
//     //         .map_err(|e| InfrastructureError::Cache(e.to_string()))?;
//     //     Ok(value)
//     // }
//     //
//     // async fn set(&self, key: &str, value: &str, expire_seconds: u64) -> ServiceResult<()> {
//     //     let mut conn = self.conn.clone();
//     //     conn.set_ex(key, value, expire_seconds)
//     //         .await
//     //         .map_err(|e| InfrastructureError::Cache(e.to_string()))?;
//     //     Ok(())
//     // }
//     //
//     // async fn delete(&self, key: &str) -> ServiceResult<()> {
//     //     let mut conn = self.conn.clone();
//     //     conn.del(key)
//     //         .await
//     //         .map_err(|e| InfrastructureError::Cache(e.to_string()))?;
//     //     Ok(())
//     // }
//     //
//     // async fn exists(&self, key: &str) -> ServiceResult<bool> {
//     //     let mut conn = self.conn.clone();
//     //     let exists: bool = conn
//     //         .exists(key)
//     //         .await
//     //         .map_err(|e| InfrastructureError::Cache(e.to_string()))?;
//     //     Ok(exists)
//     // }
//     //
//     // async fn hset(&self, key: &str, field: &str, value: &str) -> ServiceResult<()> {
//     //     let mut conn = self.conn.clone();
//     //     conn.hset(key, field, value)
//     //         .await
//     //         .map_err(|e| InfrastructureError::Cache(e.to_string()))?;
//     //     Ok(())
//     // }
//     //
//     // async fn hget(&self, key: &str, field: &str) -> ServiceResult<Option<String>> {
//     //     let mut conn = self.conn.clone();
//     //     let value: Option<String> = conn
//     //         .hget(key, field)
//     //         .await
//     //         .map_err(|e| InfrastructureError::Cache(e.to_string()))?;
//     //     Ok(value)
//     // }
//     //
//     // async fn incr(&self, key: &str) -> ServiceResult<i64> {
//     //     let mut conn = self.conn.clone();
//     //     let value: i64 = conn
//     //         .incr(key, 1)
//     //         .await
//     //         .map_err(|e| InfrastructureError::Cache(e.to_string()))?;
//     //     Ok(value)
//     // }
//     todo!{"Implement Redis cache methods"}
// }
