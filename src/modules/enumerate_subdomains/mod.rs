use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::header::USER_AGENT;

use crate::database::node::{Node, Type};
use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{events, logger};

pub struct ModuleEnumerateSubdomains {}

impl Default for ModuleEnumerateSubdomains {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleEnumerateSubdomains {
    pub fn new() -> Self {
        ModuleEnumerateSubdomains {}
    }
}

impl Module for ModuleEnumerateSubdomains {
    fn name(&self) -> String {
        String::from("enumerate:subdomains")
    }

    fn description(&self) -> String {
        String::from(
            "This module will aggressively try to find subdomains based on the given wordlist",
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
        let args = session.get_args();
        let config = session.get_config();
        let wordlist = config
            .enumerate_subdomains
            .clone()
            .wordlist
            .unwrap_or_else(|| args.wordlist.clone());
        let wordlist_file = File::open(wordlist).expect("Invalid wordlist file path");
        let lines = BufReader::new(wordlist_file).lines();
        for line in lines.map_while(Result::ok) {
            let user_agents_file = include_str!("../../../resources/user_agents.txt");
            let user_agents_lines = user_agents_file.lines();
            let random_user_agent = user_agents_lines.clone().collect::<Vec<_>>()
                [rand::thread_rng().gen_range(0..user_agents_lines.count())];

            let uri = format!("{}.{}", line, domain);
            if reqwest::blocking::Client::new()
                .get(format!("https://{}", uri))
                .header(USER_AGENT, random_user_agent)
                .send()
                .is_ok()
                && !session.get_state().has_discovered_subdomain(uri.clone())
            {
                logger::println(
                    self.name(),
                    format!("Discovered '{}' as a new subdomain", uri),
                );
                if let Some(parent) = session
                    .get_database()
                    .search(Type::Hostname, domain.clone())
                {
                    let new_node = Node::new(Type::Hostname, uri.clone());
                    parent.connect(new_node);
                }
                session.get_state().discover_subdomain(uri.clone());
                session.emit(events::Type::DiscoveredDomain(uri));
            }
        }

        Ok(())
    }
}
