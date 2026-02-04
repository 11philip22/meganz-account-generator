use thiserror::Error;

/// Errors returned by account generation operations.
#[derive(Debug, Error)]
pub enum Error {
    /// GuerrillaMail API or transport error while creating, polling, or deleting inboxes.
    #[error("GuerrillaMail error: {0}")]
    Mail(#[from] guerrillamail_client::Error),

    /// MEGA API or transport error during registration or confirmation.
    #[error("MEGA error: {0}")]
    Mega(#[from] megalib::MegaError),

    /// No MEGA confirmation email was observed before the configured timeout elapsed.
    #[error("Timeout waiting for confirmation email")]
    EmailTimeout,

    /// A likely MEGA email was found, but no usable confirmation link could be parsed.
    #[error("No confirmation link found in email")]
    NoConfirmationLink,
}

/// Crate-local result type with [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
