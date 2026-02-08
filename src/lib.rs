//! Create and confirm MEGA accounts using a temporary GuerrillaMail inbox.
//!
//! This crate drives the signup flow end-to-end:
//! 1. Generate a random GuerrillaMail alias and create a temporary address.
//! 2. Register an account with MEGA.
//! 3. Poll the inbox for a likely MEGA confirmation email.
//! 4. Extract the confirmation key from the email body and verify the registration.
//!
//! The returned [`GeneratedAccount`] is only produced after confirmation succeeds.
//!
//! # Add To `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! meganz-account-generator = "0.4"
//! ```
//!
//! # Example
//!
//! ```no_run
//! use std::time::Duration;
//! use meganz_account_generator::AccountGenerator;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let generator = AccountGenerator::builder()
//!         // Optional: route both MEGA and GuerrillaMail traffic through a proxy.
//!         // .proxy("http://127.0.0.1:8080")
//!         .timeout(Duration::from_secs(180))
//!         .poll_interval(Duration::from_secs(3))
//!         .build()
//!         .await?;
//!
//!     let account = generator
//!         .generate_with_name("S3cure-Password!", "Automation Bot")
//!         .await?;
//!
//!     println!("Created account: {}", account.email);
//!     Ok(())
//! }
//! ```
//!
//! # Behavior Notes
//!
//! - Confirmation email detection is heuristic: a message is treated as "likely MEGA" when the sender
//!   contains `"mega"` or the subject contains `"MEGA"`.
//! - If the inbox is cleaned up, it is best-effort: deletion errors are ignored after successful confirmation.
//!
//! # Errors And Timeout Semantics
//!
//! All async operations return [`Result`](crate::Result) with [`Error`](crate::Error):
//! - [`Error::Mail`]: GuerrillaMail request/transport failures while creating the address, polling the inbox,
//!   or fetching message bodies
//! - [`Error::Mega`]: MEGA request/transport failures while registering or verifying the account
//! - [`Error::EmailTimeout`]: no likely MEGA email was observed before `timeout` elapsed
//! - [`Error::NoConfirmationLink`]: a likely MEGA email was observed before `timeout`, but no confirmation key
//!   could be extracted from its body
//!
//! Polling waits `poll_interval` between inbox checks until the `timeout` elapses. The timeout is evaluated at
//! the start of each poll iteration, so total wall-clock time may exceed `timeout` by the duration of an
//! in-flight poll request plus up to one `poll_interval` sleep.
//!
//! # External Failures
//!
//! This crate depends on external services (GuerrillaMail and MEGA). Failures such as network issues, DNS/TLS
//! errors, proxy misconfiguration, service outages, or API changes surface as [`Error::Mail`] or [`Error::Mega`].
//! Email delivery is also not guaranteed; confirmation messages may be delayed, filtered, or never arrive, which
//! results in [`Error::EmailTimeout`] or [`Error::NoConfirmationLink`] depending on what was observed while polling.

mod account;
mod errors;
mod generator;
mod random;

pub use account::GeneratedAccount;
pub use errors::{Error, Result};
pub use generator::{AccountGenerator, AccountGeneratorBuilder};
