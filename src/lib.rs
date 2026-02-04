//! Create and confirm MEGA accounts with a temporary GuerrillaMail inbox.
//!
//! This crate automates the full signup flow:
//! 1. Create a random temporary email address.
//! 2. Register a MEGA account.
//! 3. Poll the inbox for the MEGA confirmation message.
//! 4. Extract the confirmation key and finalize registration.
//!
//! # Quick Start
//!
//! ```no_run
//! use meganz_account_generator::AccountGenerator;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let generator = AccountGenerator::new().await?;
//!     let account = generator.generate("S3cure-Password!").await?;
//!
//!     println!("Created account: {}", account.email);
//!     Ok(())
//! }
//! ```
//!
//! # Configuration
//!
//! Use [`AccountGenerator::builder`] to set:
//! - proxy URL for both MEGA and GuerrillaMail requests
//! - signup email timeout
//! - inbox polling interval
//!
//! ```no_run
//! use std::time::Duration;
//! use meganz_account_generator::AccountGenerator;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let generator = AccountGenerator::builder()
//!         .proxy("http://127.0.0.1:8080")
//!         .timeout(Duration::from_secs(180))
//!         .poll_interval(Duration::from_secs(3))
//!         .build()
//!         .await?;
//!
//!     let _account = generator.generate("S3cure-Password!").await?;
//!     Ok(())
//! }
//! ```
//!
//! # Errors and timeouts
//!
//! All async operations return [`Result`](crate::Result) with [`Error`](crate::Error):
//! - [`Error::Mail`] when GuerrillaMail requests fail (network/proxy issues, API errors)
//! - [`Error::Mega`] when MEGA registration or confirmation fails
//! - [`Error::EmailTimeout`] if no confirmation email arrives before the configured timeout
//! - [`Error::NoConfirmationLink`] if an email arrives but no confirmation link can be parsed
//!
//! Confirmation polling runs every `poll_interval` until `timeout` elapses.

mod account;
mod errors;
mod generator;
mod random;

pub use account::GeneratedAccount;
pub use errors::{Error, Result};
pub use generator::{AccountGenerator, AccountGeneratorBuilder};
