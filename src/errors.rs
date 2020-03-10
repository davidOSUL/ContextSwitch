use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockerError { 
    #[error("Failed to serialize")]
    FailedToSerialize,
    #[error("Failed to deserialize")]
    FailedToDeserialize
}

#[derive(Debug, Error)]
pub enum BlockError {
    #[error("Start time is after end time")]
    StartAfterEnd
}