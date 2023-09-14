use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Tag {
    BlockChain,
    Rust,
    Rebase,
    Daily,
    Other,
}

impl TryFrom<&str> for Tag {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "blockchain" => Ok(Self::BlockChain),
            "rust" => Ok(Self::Rust),
            "rebase" => Ok(Self::Rebase),
            "daily" => Ok(Self::Daily),
            _ => Ok(Self::Other),
        }
    }
}
