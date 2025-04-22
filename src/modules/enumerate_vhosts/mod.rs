use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::header::{HOST, USER_AGENT};
use serde_json::Value;

use crate::database::node::{Node, Type};
use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{events, helpers, logger};

use super::NoiseLevel;

pub struct ModuleEnumerateVhosts {}

impl Default for ModuleEnumerateVhosts {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleEnumerateVhosts {
    pub fn new() -> Self {
        ModuleEnumerateVhosts {}
    }
}

impl Module for ModuleEnumerateVhosts {
    fn name(&self) -> String {
        String::from("enumerate:vhosts")
    }

    fn description(&self) -> String {
        String::from("This module will aggressively try to find vhosts based on the given wordlist")
    }

    fn noise_level(&self) -> NoiseLevel {
        NoiseLevel::High
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
        let args = session.get_args();
        let config = session.get_config();
        let wordlist = config
            .enumerate_vhosts
            .clone()
            .wordlist
            .unwrap_or_else(|| args.wordlist.clone());
        let wordlist_file = File::open(wordlist).expect("Invalid wordlist file path");
        let lines = BufReader::new(wordlist_file).lines();
        for line in lines.map_while(Result::ok) {
            let vhost = format!("{}.{}", line, domain);
            if reqwest::blocking::Client::new()
                .get(format!("https://{}", domain))
                .header(USER_AGENT, helpers::ua::get_random())
                .header(HOST, vhost.clone())
                .send()
                .is_ok()
                && !session.get_state().has_discovered_domain(vhost.clone())
            {
                logger::println(
                    self.name(),
                    format!("Discovered '{}' as a new vhost", vhost),
                );

                if let Some(parent) = session.get_database().search(Type::Domain, domain.clone()) {
                    let mut new_node = Node::new(Type::Domain, vhost.clone());
                    if let Some(ip_addr) = helpers::network::get_ip_addr(&vhost) {
                        new_node.add_data(String::from("ip"), Value::String(ip_addr.to_string()));
                        if let Some(geoinfo) = helpers::network::geolocate_ip(ip_addr) {
                            new_node.add_data(String::from("geoinfo"), geoinfo.into())
                        }
                    }
                    parent.connect(new_node);
                }
                session.get_state().discover_domain(vhost.clone());
                session.emit(events::Type::DiscoveredDomain(vhost));
            }
        }

        Ok(())
    }
}
