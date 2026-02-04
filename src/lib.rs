//! MEGA.nz Account Generator
//!
//! Automated account registration using temporary email.

mod errors;
mod generator;

pub use errors::{Error, Result};
pub use generator::{AccountGenerator, AccountGeneratorBuilder, GeneratedAccount};
