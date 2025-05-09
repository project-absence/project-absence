use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::events;
use crate::session::Session;

pub mod banner_grabber;
pub mod domain_takeover;
pub mod dork;
pub mod enumerate_files;
pub mod enumerate_subdomains;
pub mod enumerate_vhosts;
pub mod lua_script;
pub mod passive_dns;
pub mod port_scanner;
pub mod ready;
#[cfg(feature = "chrome")]
pub mod screenshot_grabber;

pub enum Context {
    Domain(String),
    OpenPort(String, usize),
    None,
}

pub fn get_context_for_event(event: &events::Type) -> Context {
    match event {
        events::Type::DiscoveredDomain(domain) => Context::Domain(domain.clone()),
        events::Type::OpenPort(domain, port) => Context::OpenPort(domain.clone(), *port),
        _ => Context::None,
    }
}

#[derive(
    Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, ValueEnum,
)]
pub enum NoiseLevel {
    None,
    Low,
    #[default]
    Medium,
    High,
}

pub trait Module: Send + Sync {
    fn name(&self) -> String;
    #[allow(dead_code)]
    fn description(&self) -> String;
    fn subscribers(&self) -> Vec<events::Type>;
    fn execute(&self, session: &Session, context: Context) -> Result<(), String>;
}
