use crate::config::Config;
use crate::storage::Database;

pub struct HomeServer {
    config: Config,
    database: Database,
}

impl HomeServer {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let database = Database::new(&config.database).await?;
        
        Ok(Self {
            config,
            database,
        })
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Starting Axon server on {}:{}", self.config.bind_address, self.config.listen_port);
        // TODO: Implement HTTP server
        Ok(())
    }
}