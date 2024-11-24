use chrono::{Duration, Utc};
use rand::Rng;
use serde_json::Value;
use std::sync::Mutex;

use reqwest::header::USER_AGENT;

use crate::database::node::{Node, Type};
use crate::modules::passive_dns::crt_sh::CrtShItem;
use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{events, logger};

mod crt_sh;

pub struct ModulePassiveDNS {
    processed_domains: Mutex<Vec<String>>,
}

impl Default for ModulePassiveDNS {
    fn default() -> Self {
        Self::new()
    }
}

impl ModulePassiveDNS {
    pub fn new() -> Self {
        ModulePassiveDNS {
            processed_domains: Mutex::new(Vec::new()),
        }
    }

    pub fn process(&self, domain: String) {
        self.processed_domains.lock().unwrap().push(domain)
    }

    pub fn has_processed(&self, domain: String) -> bool {
        self.processed_domains.lock().unwrap().contains(&domain)
    }
}

impl Module for ModulePassiveDNS {
    fn name(&self) -> String {
        String::from("dns:passive")
    }

    fn description(&self) -> String {
        String::from(
            "This module will perform a passive discovery of new subdomains by using crt.sh",
        )
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::DiscoveredDomain(String::new())]
    }

    fn execute(&self, session: &Session, context: Context) {
        let domain = match context {
            Context::Domain(domain) => domain,
            Context::None => {
                logger::error(self.name(), "Received wrong context, exiting module");
                return;
            }
        };
        let config = session.get_config();
        let ignore_expired = config.passive_dns.ignore_expired.unwrap_or(false);
        let recent_only = config.passive_dns.recent_only.unwrap_or(false);
        if self.has_processed(domain.to_string()) {
            return;
        }
        self.process(domain.to_string());

        let file = include_str!("../../../resources/user_agents.txt");
        let lines = file.lines();
        let random_user_agent =
            lines.clone().collect::<Vec<_>>()[rand::thread_rng().gen_range(0..lines.count())];

        let response = reqwest::blocking::Client::new()
            .get(format!("https://crt.sh/?q={}&output=json", domain))
            .header(USER_AGENT, random_user_agent)
            .send();
        match response {
            Ok(response) => {
                let items: Vec<CrtShItem> = response.json().unwrap_or_default();
                for item in items {
                    let name_values = &item
                        .name_value
                        .split('\n')
                        .map(|x| x.strip_prefix("*.").unwrap_or(x).to_string())
                        .collect::<Vec<String>>();
                    for name_value in name_values {
                        if name_value == &domain.to_string() {
                            continue;
                        }
                        if !session
                            .get_state()
                            .has_discovered_subdomain(name_value.to_string())
                        {
                            let now = Utc::now();

                            // Check if the certificate has expired
                            let has_expired = item.not_after < now;
                            if ignore_expired && has_expired {
                                continue;
                            }

                            // Check if the certificate has been created within the last 24 hours
                            let is_recent = item.not_before <= now
                                && item.not_before >= now - Duration::try_hours(24).unwrap();
                            if !is_recent && recent_only {
                                continue;
                            }

                            logger::println(
                                self.name(),
                                format!(
                                    "Discovered '{}' as a new subdomain{}{}",
                                    name_value,
                                    if has_expired {
                                        " $[fg:red]$[effect:bold](Certificate expired, likely inactive)"
                                    } else {
                                        ""
                                    },
                                    if is_recent {
                                        " $[fg:blue]$[effect:bold](Active since less than 24 hours)"
                                    } else {
                                        ""
                                    }
                                ),
                            );
                            if let Some(parent) = session
                                .get_database()
                                .search(Type::Hostname, domain.clone())
                            {
                                let mut new_node = Node::new(Type::Hostname, name_value.clone());
                                new_node
                                    .add_data(String::from("expired"), Value::Bool(has_expired));
                                new_node.add_data(String::from("recent"), Value::Bool(is_recent));
                                parent.connect(new_node);
                            }
                            session
                                .get_state()
                                .discover_subdomain(name_value.to_string());
                        }
                    }
                }
            }
            Err(_) => logger::error(self.name(), "Failed performing a request to crt.sh"),
        }
    }
}
