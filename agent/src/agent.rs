use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    pin,
    task::{self, Poll},
};

use crate::{config::Config, server};

use serde::{Deserialize, Serialize};
use systemd::{journal, Journal};
use tokio::time::{self};

pub struct Logs {
    pub journal: Journal,
}

impl Logs {
    pub fn new() -> Self {
        let mut journal = journal::OpenOptions::default()
            .system(true)
            .current_user(true)
            .open()
            .unwrap();

        journal.seek(journal::JournalSeek::Head).unwrap();

        return Logs { journal };
    }

    pub async fn await_next_entry(&mut self) -> Entry {
        return Entry { logs: self };
    }
}

pub struct Entry<'a> {
    pub logs: &'a mut Logs,
}

const AWAIT_TIME_FOR_ENTRIES: time::Duration = time::Duration::from_secs(10);

impl<'a> Future for Entry<'a> {
    type Output = Option<String>;

    fn poll(mut self: pin::Pin<&mut Self>, _: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        match self
            .logs
            .journal
            .await_next_entry(Some(AWAIT_TIME_FOR_ENTRIES))
        {
            Ok(Some(entry)) => {
                let entry_as_json = serde_json::to_string(&entry).unwrap();

                return Poll::Ready(Some(entry_as_json));
            }
            Ok(None) => Poll::Pending,
            Err(_) => todo!(),
        }
    }
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
pub struct Settings {
    pub priorities: Vec<String>,
    // TODO: Add timestamp to indicate when the settings was defined on the server.
}

pub struct Agent {
    pub id: String,
    pub config: Config,
    pub settings: Option<Settings>,
    api: server::API,
}

#[derive(Debug)]
pub enum AgentError {
    ApiError(String),           // Error when calling API
    JournalError(String),       // Error related to journal operations
    LogEntryError(String),      // Error when handling log entries
    ConfigurationError(String), // Error in configuration
}

impl Display for AgentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::ApiError(msg) => write!(f, "API Error: {}", msg),
            AgentError::JournalError(msg) => write!(f, "Journal Error: {}", msg),
            AgentError::LogEntryError(msg) => write!(f, "Log Entry Error: {}", msg),
            AgentError::ConfigurationError(msg) => write!(f, "Configuration Error: {}", msg),
        }
    }
}

impl Error for AgentError {}

const INTERVAL_GET_SETTINGS: time::Duration = time::Duration::from_secs(15);

impl Agent {
    pub fn new(id: String, config: Config) -> Self {
        return Agent {
            id: id.clone(),
            config: config.clone(),
            settings: None,
            api: server::API::new(id.clone(), config.clone()),
        };
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let settings = self.api.get_settings().await?;
        self.settings = Some(settings);

        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut interval = time::interval(INTERVAL_GET_SETTINGS);
        let mut logs = Logs::new();

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    log::trace!("Ticker to update settings");

                    let new_settings = if let Ok(s) = self.api.get_settings().await { s } else {
                        log::error!("Failed to get the settings from server");

                        continue
                    };

                    if let Some(current_settings) = self.settings.clone() {
                        if !current_settings.eq(&new_settings) {
                            for priority in new_settings.priorities.iter() {
                                logs.journal
                                    .match_add("PRIORITY", String::from(priority))
                                    .map_err(|e| {
                                        AgentError::JournalError(format!("Failed to add match priority: {:?}", e))
                                    })?;
                            }

                            self.settings = Some(new_settings);
                            log::info!("Settings updated!");
                        }
                    }
                }
                entry = logs.await_next_entry().await => {
                    if let Some(entry) = entry {
                        log::debug!("{:?}", entry);

                        if let Err(e) = self.api.send_logs(&entry).await {
                            log::error!("Failed to send logs: {e}");
                        } else {
                            log::info!("Logs sent");
                        }
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct JournalEntry {
    message: String,
    priority: String,

    #[serde(rename = "__CURSOR")]
    cursor: String,

    #[serde(rename = "__REALTIME_TIMESTAMP")]
    realtime_timestamp: String,

    #[serde(rename = "__MONOTONIC_TIMESTAMP")]
    monotonic_timestamp: String,

    #[serde(rename = "_BOOT_ID")]
    boot_id: String,

    #[serde(rename = "_TRANSPORT")]
    transport: String,

    #[serde(rename = "SYSLOG_FACILITY")]
    syslog_facility: String,

    #[serde(rename = "SYSLOG_IDENTIFIER")]
    syslog_identifier: String,

    #[serde(rename = "_UID")]
    uid: String,

    #[serde(rename = "_GID")]
    gid: String,

    #[serde(rename = "_COMM")]
    comm: String,

    #[serde(rename = "_EXE")]
    exe: String,

    #[serde(rename = "_CMDLINE")]
    cmdline: String,

    #[serde(rename = "_SYSTEMD_CGROUP")]
    systemd_cgroup: String,

    #[serde(rename = "_SYSTEMD_SESSION")]
    systemd_session: String,

    #[serde(rename = "_SYSTEMD_OWNER_UID")]
    systemd_owner_uid: String,

    #[serde(rename = "_SYSTEMD_UNIT")]
    systemd_unit: String,

    #[serde(rename = "_SYSTEMD_SLICE")]
    systemd_slice: String,

    #[serde(rename = "_SYSTEMD_USER_SLICE")]
    systemd_user_slice: String,

    #[serde(rename = "_PID")]
    pid: String,
}
