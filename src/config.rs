use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::logger;

const DEFAULT_CONFIG: &str = r#"[banner_grabber]
enabled = false

[dork]
enabled = false

[enumerate_files]
enabled = false

[enumerate_subdomains]
enabled = false

[passive_dns]
enabled = false

[port_scanner]
enabled = false

[screenshot_grabber]
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
    pub banner_grabber: BannerGrabberConfig,
    pub dork: DorkConfig,
    pub enumerate_files: EnumerateFilesConfig,
    pub enumerate_subdomains: EnumerateSubdomainsConfig,
    pub passive_dns: PassiveDNSConfig,
    pub port_scanner: PortScannerConfig,
    #[cfg(feature = "chrome")]
    pub screenshot_grabber: ScreenshotGrabberConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BannerGrabberConfig {
    /// Whether the module is enabled
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DorkConfig {
    /// Whether the module is enabled
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnumerateFilesConfig {
    /// Whether the module is enabled
    pub enabled: bool,
    /// The path to the wordlist to use
    pub wordlist: Option<String>,
    /// The extension to append to the file names
    pub files_extension: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnumerateSubdomainsConfig {
    /// Whether the module is enabled
    pub enabled: bool,
    /// The path to the wordlist to use
    pub wordlist: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortScannerConfig {
    /// Whether the module is enabled
    pub enabled: bool,
    /// The range of port to scan for (always inclusive)
    /// Examples:
    /// * `1-20` -> Port 1 to 20
    /// * `1-20,22,40-60` -> Port 1 to 20, port 22 and port 40 to 60
    ///
    /// If this is option is not provided, it will use the top 1000 most common ports of nmap
    pub range: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenshotGrabberConfig {
    /// Whether the module is enabled
    pub enabled: bool,
    /// Path for Chrome or Chromium
    /// If unspecified, it will try to automatically detect a suitable binary
    pub chrome_path: Option<String>,
    /// Whether the screenshots should be saved as separate file
    /// If false, it will base64 encode the screenshot and save it in the JSON file
    pub save_as_file: Option<bool>,
}
