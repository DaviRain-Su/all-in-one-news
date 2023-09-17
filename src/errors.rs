use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("dummy error")]
    Dummy,
}
