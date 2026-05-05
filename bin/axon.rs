use clap::Parser;
use axon::config::Config;
use axon::server::HomeServer;

#[derive(Parser, Debug)]
#[command(name = "axon")]
#[command(about = "High-performance Matrix homeserver", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let config_content = std::fs::read_to_string(&args.config)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    
    let server = HomeServer::new(config).await?;
    server.run().await?;
    
    Ok(())
}