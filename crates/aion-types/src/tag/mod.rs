use serde::{Deserialize, Serialize};

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
