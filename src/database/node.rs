use std::{collections::HashMap, fmt};

use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::{flags, modules::port_scanner::OpenPort};

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Hostname,
    File,
}

impl fmt::Display for Type {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Hostname => {
                write!(formatter, "hostname")
            }
            Type::File => {
                write!(formatter, "file")
            }
        }
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Node {
    r#type: Type,
    value: String,
    connections: Vec<Node>,
    data: HashMap<String, Value>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node {{ type: {}, value: {}, connections: {}, data: {:?} }}",
            self.r#type,
            self.value,
            self.get_connections().len(),
            self.data
        )
    }
}

impl Node {
    pub fn new(r#type: Type, value: String) -> Self {
        Node {
            r#type,
            value,
            connections: Vec::new(),
            data: HashMap::new(),
        }
    }

    pub fn connect(&mut self, node: Node) {
        self.connections.push(node);
    }

    #[allow(unused)]
    pub fn add(&mut self, r#type: Type, value: String) {
        self.connect(Node::new(r#type, value));
    }

    pub fn add_data(&mut self, key: String, value: Value) {
        self.data.insert(key, value);
    }

    #[allow(dead_code)]
    pub fn get_data(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    #[allow(dead_code)]
    pub fn edit_data(&mut self, key: String, new_value: Value) {
        self.data.entry(key).and_modify(|e| *e = new_value);
    }

    pub fn get_or_init_map(&mut self, key: &str) -> serde_json::Map<String, Value> {
        if !self.data.contains_key(key) {
            self.data
                .insert(key.to_string(), Value::Object(serde_json::Map::new()));
        }
        self.get_data(key)
            .and_then(|value| value.as_object().cloned())
            .expect("JSON object should exist")
    }

    pub fn get_connections(&self) -> Vec<Node> {
        self.connections.clone()
    }

    pub fn find(&mut self, node: &Node) -> Option<&mut Node> {
        if self.equals(node) {
            return Some(self);
        }

        for connection in &mut self.connections {
            if connection.equals(node) {
                return Some(connection);
            } else if let Some(found) = connection.find(node) {
                return Some(found);
            }
        }

        None
    }

    pub fn equals(&self, other: &Node) -> bool {
        // Check type and value
        if self.r#type != other.r#type || self.value != other.value {
            return false;
        }

        // Check if all the connections are the same
        for (self_connection, other_connection) in self.connections.iter().zip(&other.connections) {
            if !self_connection.equals(other_connection) {
                return false;
            }
        }

        true
    }

    pub fn to_markdown(&self) -> String {
        let banners = if let Some(banners) = self.get_data("banners") {
            let mut result = String::from("#### Banners");
            let obj = banners.as_object().unwrap();
            for (port, data) in obj {
                let banner_data = data.as_object().unwrap();
                result += format!("\n\n**Port {}:**\n", port).as_str();
                for (title, value) in banner_data {
                    result += format!("\n- {}: {}", title, value).as_str();
                }
            }
            Some(result)
        } else {
            None
        };
        let flags = if let Some(flags) = self.get_data("flags") {
            let mut result = String::from("#### Flags\n");
            result += format!(
                "\n- `IS_RECENT` => {}\n- `HAS_EXPIRED` => {}",
                flags::contains_to_markdown(
                    flags.as_u64().unwrap() as usize,
                    flags::hostname::IS_RECENT
                ),
                flags::contains_to_markdown(
                    flags.as_u64().unwrap() as usize,
                    flags::hostname::HAS_EXPIRED
                )
            )
            .as_str();
            Some(result)
        } else {
            None
        };
        let open_ports = if let Some(ports) = self.get_data("ports") {
            let mut result = String::from("#### Open Ports\n");
            let arr = ports.as_array().unwrap();
            for port in arr {
                let open_port: OpenPort = serde_json::from_value(port.clone()).unwrap();
                result += format!(
                    "\n- **{}:** {}",
                    open_port.port, open_port.potential_service
                )
                .as_str();
            }
            Some(result)
        } else {
            None
        };

        let connections_markdown: String = self
            .get_connections()
            .iter()
            .map(|conn| conn.to_markdown())
            .collect::<Vec<_>>()
            .join("\n\n");

        let mut sections = vec![format!("### {}", self.value)];
        if let Some(banners) = banners {
            sections.push(banners);
        }
        if let Some(flags) = flags {
            sections.push(flags);
        }
        if let Some(open_ports) = open_ports {
            sections.push(open_ports);
        }
        if !connections_markdown.is_empty() {
            sections.push(connections_markdown);
        }
        sections.join("\n\n")
    }
}
