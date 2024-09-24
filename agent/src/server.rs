use std::error::Error;

use crate::{agent, config::Config};

use reqwest::{header::HeaderMap, Client};

#[derive(Clone, Debug)]
pub struct API {
    id: String,
    host: String,
    client: Client,
}

// TODO: Add version on the user agent.
const LOGRERO_USER_AGENT: &str = "logrero";

impl API {
    pub fn new(id: String, config: Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            format!("Bearer {}", config.server.token).parse().unwrap(),
        );

        return API {
            id,
            host: format!("http://{}:{}", config.server.address, config.server.port),
            client: Client::builder()
                .default_headers(headers)
                .user_agent(LOGRERO_USER_AGENT)
                .gzip(true)
                .build()
                .unwrap(),
        };
    }

    pub async fn get_settings(&self) -> Result<agent::Settings, Box<dyn Error>> {
        let request = self
            .client
            .get(format!("{}/api/v1/device/{}/settings", self.host, self.id));

        let response = request.send().await?;

        if !response.status().is_success() {
            log::error!("Failed to get the settings: HTTP {}", response.status());

            // TODO: Return error here.
        }

        let settings = response.json::<agent::Settings>().await?;

        log::trace!("Settings successfully got from the server");

        return Ok(settings);
    }

    pub async fn send_logs(&self, entry: &String) -> Result<(), Box<dyn Error>> {
        let request = self
            .client
            .post(format!("{}/api/v1/device/{}/logs", self.host, self.id))
            .json(entry);

        let response = request.send().await?;

        if !response.status().is_success() {
            log::error!("Failed to send logs: HTTP {}", response.status());

            // TODO: Return error here.
        }

        log::trace!("Logs successfully sent to the server.");

        return Ok(());
    }
}
