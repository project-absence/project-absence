use std::{collections::HashMap, fmt};

use serde::{Serialize, Serializer};
use serde_json::Value;

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
}
