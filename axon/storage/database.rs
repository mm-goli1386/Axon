use crate::config::DatabaseConfig;
use sqlx::{PgPool, SqlitePool, Pool, Any};
use std::sync::Arc;

pub enum Database {
    Postgres(Pool<sqlx::Postgres>),
    Sqlite(Pool<sqlx::Sqlite>),
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, sqlx::Error> {
        match config {
            DatabaseConfig::Postgres { url, pool_size } => {
                let pool = PgPool::connect(url).await?;
                Ok(Database::Postgres(pool))
            }
            DatabaseConfig::Sqlite { path } => {
                let pool = SqlitePool::connect(path).await?;
                Ok(Database::Sqlite(pool))
            }
        }
    }
}

pub trait DataProvider: Send + Sync {
    async fn get_user(&self, user_id: &str) -> Result<Option<serde_json::Value>, sqlx::Error>;
}