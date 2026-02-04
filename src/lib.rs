//! MEGA.nz Account Generator
//!
//! Automated account registration using temporary email.

mod account;
mod errors;
mod generator;
mod random;

pub use account::GeneratedAccount;
pub use errors::{Error, Result};
pub use generator::{AccountGenerator, AccountGeneratorBuilder};
