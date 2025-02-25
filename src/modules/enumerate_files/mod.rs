use reqwest::StatusCode;
use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::header::USER_AGENT;

use crate::database::node::{Node, Type};
use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{events, helpers, logger};

use super::NoiseLevel;

pub struct ModuleEnumerateFiles {}

impl Default for ModuleEnumerateFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleEnumerateFiles {
    pub fn new() -> Self {
        ModuleEnumerateFiles {}
    }
}

impl Module for ModuleEnumerateFiles {
    fn name(&self) -> String {
        String::from("enumerate:files")
    }

    fn description(&self) -> String {
        String::from(
            "This module will aggressively try to find files based on the given wordlist and extension",
        )
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
            .enumerate_files
            .clone()
            .wordlist
            .unwrap_or_else(|| args.wordlist.clone());
        let wordlist_file = File::open(wordlist).expect("Invalid wordlist file path");
        let extension = config
            .enumerate_files
            .clone()
            .files_extension
            .unwrap_or_else(|| "php".to_string());
        let lines = BufReader::new(wordlist_file).lines();
        for line in lines.map_while(Result::ok) {
            let uri = format!(
                "{}/{}{}",
                domain,
                line,
                if !extension.is_empty() {
                    format!(".{}", extension)
                } else {
                    "".to_string()
                }
            );
            if let Ok(response) = reqwest::blocking::Client::new()
                .get(format!("https://{}", uri))
                .header(USER_AGENT, helpers::ua::get_random())
                .send()
            {
                if response.status() == StatusCode::OK {
                    logger::println(
                        self.name(),
                        format!("Discovered '{}' as an existing file", uri),
                    );
                    if let Some(parent) =
                        session.get_database().search(Type::Domain, domain.clone())
                    {
                        let new_node = Node::new(Type::File, uri.clone());
                        parent.connect(new_node);
                    }
                }
            }
        }

        Ok(())
    }
}
