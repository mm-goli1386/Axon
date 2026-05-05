use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "backend")]
pub enum DatabaseConfig {
    Postgres {
        url: String,
        pool_size: u32,
    },
    Sqlite {
        path: String,
    },
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig::Sqlite {
            path: "axon.db".to_string(),
        }
    }
}