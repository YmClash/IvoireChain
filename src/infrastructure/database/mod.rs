//! Database connection pool management.

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::Config;
use crate::shared::errors::InfrastructureError;


/// Create a new PostgreSQL connection pool
pub async  fn  create_pool(config: &Config) -> Result<PgPool,InfrastructureError> {
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
        .map_err(|e| InfrastructureError::Connection(format!("Failed to connect to database: {}", e)))?;

    tracing::info!("PostgresSQL Pool created with {} max connections", config.database_max_connections);
    Ok(pool)
}


/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), InfrastructureError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| InfrastructureError::Database(sqlx::Error::Migrate(Box::new(e))))?;

    tracing::info!("Database migrations completed successfully");
    Ok(())
}

