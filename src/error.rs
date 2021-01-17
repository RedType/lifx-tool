use thiserror::Error;

#[derive(Debug, Error)]
pub enum LifxError {
    #[error("Header contains unexpected values")]
    MalformedHeader,
    #[error("{0} is not a known message type")]
    UnknownMessageType(u16),
    #[error("Buffer is {found} bytes, expected at least {expected} bytes")]
    WrongSize { found: usize, expected: usize },
}
