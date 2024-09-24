use std::{error::Error, fs};

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub token: String,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: Server,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let config_content = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_content)?;

        Ok(config)
    }
}

pub const DEFAULT_CONFIG_PATH: &'static str = &"config.toml";
