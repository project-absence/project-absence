use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{logger, modules::dork};

const DEFAULT_CONFIG: &str = r#"[domain_takeover]
enabled = false

[dork]
enabled = false

[passive_dns]
enabled = false
"#;

pub fn create_file_if_not_existing() {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| String::from(""));
    let path = PathBuf::from(format!("{}/.absence/config.toml", home_dir));
    if !path.exists() {
        if let Some(parent) = path.parent() {
            if fs::create_dir_all(parent).is_err() {
                logger::error(
                    "setup",
                    "Failed creating the directories for the default config file",
                );
            }
            if fs::write(path, DEFAULT_CONFIG).is_err() {
                logger::error(
                    "setup",
                    "Failed writing the default content of the config file",
                );
            }
        }
    }
}

/// The config.toml file structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub domain_takeover: Option<DomainTakeoverConfig>,
    pub dork: Option<DorkConfig>,
    pub passive_dns: Option<PassiveDNSConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainTakeoverConfig {
    /// Whether the module is enabled
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DorkConfig {
    /// Whether the module is enabled
    pub enabled: bool,
    /// The search engine to use
    pub search_engine: Option<dork::SearchEngine>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PassiveDNSConfig {
    /// Whether the module is enabled
    pub enabled: bool,
    /// Ignore expired certificates
    pub ignore_expired: Option<bool>,
    /// Only care about the recently (24 hours) created certificates
    pub recent_only: Option<bool>,
}
