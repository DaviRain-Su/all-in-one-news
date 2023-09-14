use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid subscriber name Error: {0}")]
    ValidateSubscriberNameError(String),
    #[error("invalid subscriber email Error: {0}")]
    ValidateSubscriberEmailError(String),
}
