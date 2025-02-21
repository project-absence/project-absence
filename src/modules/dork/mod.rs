use regex::Regex;
use reqwest::header::USER_AGENT;
use serde_json::Value;

use crate::database::node::{Node, Type};
use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{events, helpers, logger};

use super::NoiseLevel;

// At the moment just Google, so that's just a static string to append to the logger
// Will add other engines later such as Yandex, Yahoo, etc.
const MODE: &str = "google";

pub struct ModuleDork {}

impl Default for ModuleDork {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleDork {
    pub fn new() -> Self {
        ModuleDork {}
    }
}

impl Module for ModuleDork {
    fn name(&self) -> String {
        format!("dork:{}", MODE)
    }

    fn noise_level(&self) -> NoiseLevel {
        NoiseLevel::None
    }

    fn description(&self) -> String {
        String::from(
            "Uses search operators and terms on search engines to try gather more information",
        )
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

        let uri = format!("https://www.google.com/search?q=site%3A{0}", domain);
        if let Ok(response) = session
            .get_http_client()
            .get(uri.clone())
            // https://github.com/benbusby/whoogle-search/issues/1211
            .header(
                USER_AGENT,
                "Lynx/2.9.2 libwww-FM/2.14 SSL-MM/1.4.1 OpenSSL/3.4.0",
            )
            .send()
        {
            let html = response.text().unwrap_or_default();
            let re = Regex::new(&format!(
                r#"href="/url\?q=https://([a-zA-Z0-9.-]+\.{})"#,
                regex::escape(&domain)
            ))
            .unwrap();
            for cap in re.captures_iter(&html) {
                if let Some(subdomain) = cap.get(1) {
                    let subdomain = subdomain.as_str();
                    if !session
                        .get_state()
                        .has_discovered_subdomain(subdomain.to_string())
                    {
                        logger::println(
                            self.name(),
                            format!("Discovered '{}' as a new subdomain", subdomain),
                        );

                        if let Some(parent) = session
                            .get_database()
                            .search(Type::Hostname, domain.clone())
                        {
                            let mut new_node = Node::new(Type::Hostname, subdomain.to_string());
                            if let Some(ip_addr) = helpers::network::get_ip_addr(subdomain) {
                                new_node.add_data(
                                    String::from("ip"),
                                    Value::String(ip_addr.to_string()),
                                );
                                if let Some(geoinfo) = helpers::network::geolocate_ip(ip_addr) {
                                    new_node.add_data(String::from("geoinfo"), geoinfo.into())
                                }
                            }
                            parent.connect(new_node);
                        }
                        session
                            .get_state()
                            .discover_subdomain(subdomain.to_string());
                        session.emit(events::Type::DiscoveredDomain(subdomain.to_string()));
                    }
                }
            }
            Ok(())
        } else {
            Err("Unable to reach Google".to_string())
        }
    }
}
