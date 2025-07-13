use std::env;
use std::fs::File;
use std::fs::create_dir_all;
use std::io::Write;
use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose;
use headless_chrome::Browser;
use headless_chrome::LaunchOptions;
use headless_chrome::browser;
use headless_chrome::protocol::cdp::Page;
use serde_json::Value;

use crate::config;
use crate::database::node::Type;
use crate::events;
use crate::logger;
use crate::modules::{Context, Module};
use crate::session::Session;

pub struct ModuleScreenshotGrabber {
    config: config::ScreenshotGrabberConfig,
}

impl ModuleScreenshotGrabber {
    pub fn new(config: config::ScreenshotGrabberConfig) -> Self {
        ModuleScreenshotGrabber { config }
    }
}

impl Module for ModuleScreenshotGrabber {
    fn name(&self) -> String {
        String::from("grabber:screenshot")
    }

    fn description(&self) -> String {
        String::from("This module will take screenshots of newly discovered domains")
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::DiscoveredDomain(String::new())]
    }

    fn execute(&self, session: &Session, context: Context) -> Result<(), String> {
        let domain = match context {
            Context::Domain(domain) => domain,
            _ => {
                return Err("Received wrong context, exiting module".to_string());
            }
        };

        let chrome_path = if let Some(chrome_path) = &self.config.chrome_path {
            Some(PathBuf::from(chrome_path))
        } else {
            browser::default_executable().ok()
        };

        let browser = Browser::new(
            LaunchOptions::default_builder()
                .path(chrome_path)
                .window_size(Some((1920, 1080)))
                .build()
                .unwrap(),
        )
        .map_err(|e| e.to_string())?;
        let tab = browser.new_tab().map_err(|e| e.to_string())?;
        tab.navigate_to(&format!("https://{}", domain))
            .map_err(|e| e.to_string())?;
        let screenshot = tab
            .wait_for_element("html")
            .map_err(|e| e.to_string())?
            .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)
            .map_err(|e| e.to_string())?;

        let save_data = if self.config.save_as_file.is_some_and(|value| value) {
            let home_dir = env::var("HOME")
                .or_else(|_| env::var("USERPROFILE"))
                .unwrap_or(String::from(""));
            let result_path = &session.get_args().output;
            let expanded_result_path = if result_path.starts_with("~") {
                let mut expanded_path = result_path.clone();
                expanded_path.replace_range(0..1, &home_dir);
                expanded_path
            } else {
                result_path.clone()
            };
            let screenshot_path = PathBuf::from(format!(
                "{}/screenshots/{}.png",
                expanded_result_path, domain
            ));
            if create_dir_all(screenshot_path.parent().unwrap()).is_ok() {
                let mut file_result =
                    File::create(screenshot_path.clone()).map_err(|e| e.to_string())?;
                file_result
                    .write_all(&screenshot)
                    .map_err(|e| e.to_string())?;
            }
            screenshot_path.to_string_lossy().to_string()
        } else {
            general_purpose::STANDARD.encode(&screenshot)
        };

        if let Some(parent) = session.get_database().search(Type::Domain, domain.clone()) {
            parent.add_data(String::from("screenshot"), Value::String(save_data));
            logger::println(
                self.name(),
                format!(
                    "Successfully grabbed a screenshot for the domain '{}'",
                    domain
                ),
            )
        }

        Ok(())
    }
}
