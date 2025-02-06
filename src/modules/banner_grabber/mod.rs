use std::collections::HashMap;

use http::HttpBannerGrabber;
use reqwest::blocking::Client;

use crate::database::node::Type;
use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{events, logger};

use super::NoiseLevel;

mod http;

trait BannerGrabber {
    fn grab_banner(&self, http_client: &Client) -> Banner;
}

#[derive(Debug)]
pub struct Banner(pub HashMap<String, String>);

impl Banner {
    pub fn to_json_value(&self) -> serde_json::Value {
        let mut banner_map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        for (k, v) in &self.0 {
            banner_map.insert(k.clone(), serde_json::Value::String(v.clone()));
        }
        serde_json::Value::Object(banner_map)
    }
}

pub struct ModuleBannerGrabber {}

impl Default for ModuleBannerGrabber {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleBannerGrabber {
    pub fn new() -> Self {
        ModuleBannerGrabber {}
    }
}

impl Module for ModuleBannerGrabber {
    fn name(&self) -> String {
        String::from("grabber:banner")
    }

    fn noise_level(&self) -> NoiseLevel {
        NoiseLevel::Medium
    }

    fn description(&self) -> String {
        String::from("This module will parse relevant information about a running service on ports")
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::OpenPort(String::new(), 0)]
    }

    fn execute(&self, session: &Session, context: Context) -> Result<(), String> {
        let (hostname, port) = match context {
            Context::OpenPort(hostname, port) => (hostname, port),
            _ => {
                return Err("Received wrong context, exiting module".to_string());
            }
        };

        let grabber: Box<dyn BannerGrabber> = match port {
            80 | 443 => Box::new(HttpBannerGrabber::new(hostname.clone(), port)),
            _ => {
                logger::info(
                    self.name(),
                    format!("Banner grabber for port {} has not been implemented.", port),
                );
                return Ok(());
            }
        };

        let banner = grabber.grab_banner(session.get_http_client());
        if let Some(parent) = session.get_database().search(Type::Hostname, hostname) {
            let banners = parent.get_or_init_map("banners");
            let mut updated_banners = banners.clone();
            updated_banners.insert(port.to_string(), banner.to_json_value());
            parent.add_data(
                String::from("banners"),
                serde_json::Value::Object(updated_banners),
            );
            logger::println(
                self.name(),
                format!("Successfully grabbed a banner for the port {}", port),
            )
        }

        Ok(())
    }
}
