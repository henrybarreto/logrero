use std::error::Error;

mod agent;
mod config;
mod server;

use agent::Agent;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    log::info!("Starting Logrero Agent...");

    let id = String::from("device");
    let config = Config::from_file(config::DEFAULT_CONFIG_PATH)?;

    let mut agent = Agent::new(id, config);

    if let Err(e) = agent.init().await {
        log::error!("Failed to get the initial log configuration from the server");

        return Err(e);
    }

    if let Err(_) = agent.run().await {
        log::error!("Failed during the Agent's running");
    }

    log::info!("Logrero Agent has been shut down");

    Ok(())
}
