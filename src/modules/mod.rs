use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::events;
use crate::session::Session;

pub mod banner_grabber;
pub mod dork;
pub mod enumerate_files;
pub mod enumerate_subdomains;
pub mod lua_script;
pub mod passive_dns;
pub mod port_scanner;
pub mod ready;
pub mod screenshot_grabber;

pub enum Context {
    Domain(String),
    OpenPort(String, u16),
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
    fn noise_level(&self) -> NoiseLevel;
    fn subscribers(&self) -> Vec<events::Type>;
    fn execute(&self, session: &Session, context: Context) -> Result<(), String>;
}
