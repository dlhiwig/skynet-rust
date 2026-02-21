use skynet_rust::{Config, Result, SkynetAgent};
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("🤖 SKYNET-RUST v{} starting...", skynet_rust::VERSION);

    // Load configuration
    let config = Config::load().await?;
    info!("Configuration loaded successfully");

    // Create and initialize the agent
    let mut agent = SkynetAgent::new(config).await?;
    info!("SKYNET agent initialized");

    // Start the main agent loop
    info!("🚀 Starting agent loop...");
    match agent.run().await {
        Ok(_) => info!("Agent shutdown gracefully"),
        Err(e) => {
            error!("Agent error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}