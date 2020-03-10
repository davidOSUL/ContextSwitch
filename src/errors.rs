use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockerError { 
    #[error("Site already blocked")]
    SiteAlreadyBlocked,
    #[error("Site already unblocked")]
    SiteAlreadyUnblocked,
    #[error("Failed to serialize")]
    FailedToSerialize,
    #[error("Failed to deserialize")]
    FailedToDeserialize
}