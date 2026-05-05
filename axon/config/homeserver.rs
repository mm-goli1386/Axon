use serde::{Deserialize, Serialize};
use super::database::DatabaseConfig;
use super::logging::LoggingConfig;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server_name: String,
    pub listen_port: u16,
    pub bind_address: String,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub registration_enabled: bool,
    pub max_upload_size_mb: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_name: "localhost".to_string(),
            listen_port: 8008,
            bind_address: "0.0.0.0".to_string(),
            database: DatabaseConfig::default(),
            logging: LoggingConfig::default(),
            registration_enabled: true,
            max_upload_size_mb: 50,
        }
    }
}