use thiserror::Error;

/// Errors returned by account generation operations.
#[derive(Debug, Error)]
pub enum Error {
    /// GuerrillaMail API or transport error.
    ///
    /// This covers failures while creating the temporary address, polling the inbox, or fetching message bodies.
    ///
    /// Note: inbox deletion is attempted on a best-effort basis after successful confirmation, and deletion
    /// failures are ignored (they are not surfaced as an error).
    #[error("GuerrillaMail error: {0}")]
    Mail(#[from] guerrillamail_client::Error),

    /// MEGA API or transport error.
    ///
    /// This covers failures during account registration and during verification/confirmation.
    #[error("MEGA error: {0}")]
    Mega(#[from] megalib::MegaError),

    /// No likely MEGA confirmation email was observed before the configured timeout elapsed.
    #[error("Timeout waiting for confirmation email")]
    EmailTimeout,

    /// A likely MEGA email was observed, but no confirmation key could be extracted from its body.
    #[error("No confirmation link found in email")]
    NoConfirmationLink,
}

/// Crate-local result type.
pub type Result<T> = std::result::Result<T, Error>;
