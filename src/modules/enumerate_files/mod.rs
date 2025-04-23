use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::header::USER_AGENT;

use crate::database::node::{Node, Type};
use crate::modules::{Context, Module};
use crate::session::Session;
use crate::{config, events, helpers, logger};

use super::NoiseLevel;

pub struct ModuleEnumerateFiles {
    config: config::EnumerateFilesConfig,
    match_status: Vec<usize>,
}

impl ModuleEnumerateFiles {
    pub fn new(config: config::EnumerateFilesConfig) -> Self {
        ModuleEnumerateFiles {
            config,
            match_status: Vec::from([
                200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215,
                216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231,
                232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247,
                248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263,
                264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279,
                280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295,
                296, 297, 298, 299, 301, 302, 307, 401, 403, 405, 500,
            ]),
        }
    }

    pub fn noise_level() -> NoiseLevel {
        NoiseLevel::High
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

        let match_status = if let Some(match_status_range) = &self.config.match_status {
            helpers::parsing::parse_range(match_status_range.as_str())
        } else {
            self.match_status.clone()
        };

        let wordlist = self.config.wordlist.as_deref().unwrap_or(&args.wordlist);
        let wordlist_file = File::open(wordlist).expect("Invalid wordlist file path");
        let extension = self.config.files_extension.as_deref().unwrap_or("php");
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
                let status_code = response.status();
                if match_status.contains(&(status_code.as_u16() as usize)) {
                    logger::println(
                        self.name(),
                        format!("Discovered '{}' ({}) as an existing file", uri, status_code),
                    );
                    if let Some(parent) =
                        session.get_database().search(Type::Domain, domain.clone())
                    {
                        parent.connect(Node::new(Type::File, uri.clone()));
                    }
                }
            }
        }

        Ok(())
    }
}
