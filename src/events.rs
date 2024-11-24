use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Ready,
    FinishedTask,
    /// String: the discovered domain
    DiscoveredDomain(String),
}

impl fmt::Display for Type {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Ready => {
                write!(formatter, "ready")
            }
            Type::FinishedTask => {
                write!(formatter, "finished:task")
            }
            Type::DiscoveredDomain(_) => {
                write!(formatter, "discovered:domain")
            }
        }
    }
}
