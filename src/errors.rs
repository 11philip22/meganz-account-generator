use thiserror::Error;

/// Error types for account generation.
#[derive(Debug, Error)]
pub enum Error {
    #[error("GuerrillaMail error: {0}")]
    Mail(#[from] guerrillamail_client::Error),

    #[error("MEGA error: {0}")]
    Mega(#[from] megalib::MegaError),

    #[error("Timeout waiting for confirmation email")]
    EmailTimeout,

    #[error("No confirmation link found in email")]
    NoConfirmationLink,
}

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;
