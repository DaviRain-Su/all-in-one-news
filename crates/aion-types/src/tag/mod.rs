use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
pub enum Tag {
    BlockChain,
    Rust,
    Rebase,
    Daily,
    Other(String),
}

impl TryFrom<&str> for Tag {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "blockchain" => Ok(Self::BlockChain),
            "rust" => Ok(Self::Rust),
            "rebase" => Ok(Self::Rebase),
            "daily" => Ok(Self::Daily),
            other => Ok(Self::Other(other.to_string())),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlockChain => write!(f, "blockchain"),
            Self::Rust => write!(f, "rust"),
            Self::Rebase => write!(f, "rebase"),
            Self::Daily => write!(f, "daily"),
            Self::Other(other) => write!(f, "{}", other),
        }
    }
}
