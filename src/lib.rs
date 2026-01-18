//! MEGA.nz Account Generator
//!
//! Automated account registration using temporary email.

use guerrillamail_client::Client as MailClient;
use megalib::{register, verify_registration};
use rand::Rng;
use regex::Regex;
use std::time::Duration;
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

    #[error("Invalid confirmation link format")]
    InvalidConfirmationLink,
}

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;

/// A generated MEGA account.
#[derive(Debug, Clone)]
pub struct GeneratedAccount {
    /// The email address used for registration.
    pub email: String,
    /// The account password.
    pub password: String,
    /// The account holder's name.
    pub name: String,
}

impl std::fmt::Display for GeneratedAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Email: {}\nPassword: {}\nName: {}",
            self.email, self.password, self.name
        )
    }
}

/// Account generator that combines GuerrillaMail and MEGA.
pub struct AccountGenerator {
    mail_client: MailClient,
    timeout: Duration,
    poll_interval: Duration,
    proxy: Option<String>,
}

impl AccountGenerator {
    /// Create a new account generator.
    pub async fn new(proxy: Option<&str>) -> Result<Self> {
        let mail_client = MailClient::with_proxy(proxy).await?;
        Ok(Self {
            mail_client,
            timeout: Duration::from_secs(300), // 5 minute timeout
            poll_interval: Duration::from_secs(5),
            proxy: proxy.map(String::from),
        })
    }

    /// Create a new account generator with custom timeouts.
    /// Create a new account generator with custom timeouts.
    pub async fn with_timeouts(
        timeout: Duration,
        poll_interval: Duration,
        proxy: Option<&str>,
    ) -> Result<Self> {
        let mail_client = MailClient::with_proxy(proxy).await?;
        Ok(Self {
            mail_client,
            timeout,
            poll_interval,
            proxy: proxy.map(String::from),
        })
    }

    /// Generate a MEGA account.
    ///
    /// # Arguments
    /// * `password` - The password for the new account
    /// * `name` - Optional name (random if not provided)
    pub async fn generate(&self, password: &str, name: Option<&str>) -> Result<GeneratedAccount> {
        // Generate random alias
        let alias = generate_random_alias();
        let account_name = name.map(String::from).unwrap_or_else(generate_random_name);

        let email = self.mail_client.create_email(&alias, None).await?;

        let state = register(&email, password, &account_name, self.proxy.as_deref()).await?;

        // Poll for confirmation email
        let confirm_key = self.wait_for_confirmation(&email).await?;

        verify_registration(&state, &confirm_key, self.proxy.as_deref()).await?;

        // Cleanup: delete temporary email
        let _ = self.mail_client.delete_email(&email).await;

        Ok(GeneratedAccount {
            email,
            password: password.to_string(),
            name: account_name,
        })
    }

    /// Wait for the MEGA confirmation email and extract the signup key.
    async fn wait_for_confirmation(&self, email: &str) -> Result<String> {
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() >= self.timeout {
                return Err(Error::EmailTimeout);
            }

            let messages = self.mail_client.get_messages(email).await?;

            // Look for MEGA confirmation email
            for msg in &messages {
                if msg.mail_from.contains("mega") || msg.mail_subject.contains("MEGA") {
                    // Fetch full email body
                    let details = self.mail_client.fetch_email(email, &msg.mail_id).await?;
                    if let Some(key) = extract_confirm_key(&details.mail_body) {
                        return Ok(key);
                    }
                }
            }

            tokio::time::sleep(self.poll_interval).await;
        }
    }
}

/// Extract the confirmation key from a MEGA email body.
fn extract_confirm_key(body: &str) -> Option<String> {
    // MEGA confirmation links look like:
    // https://mega.nz/#confirm<KEY>
    // https://mega.nz/confirm<KEY>

    let patterns = [
        r"https://mega\.nz/#confirm([a-zA-Z0-9_-]+)",
        r"https://mega\.nz/confirm([a-zA-Z0-9_-]+)",
        r#"href="https://mega\.nz/#confirm([^"]+)"#,
        r#"href="https://mega\.nz/confirm([^"]+)"#,
    ];

    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(body) {
                if let Some(key) = caps.get(1) {
                    return Some(key.as_str().to_string());
                }
            }
        }
    }

    None
}

/// Generate a random email alias.
fn generate_random_alias() -> String {
    let mut rng = rand::thread_rng();
    let adjectives = [
        "cool", "fast", "smart", "happy", "lucky", "mega", "super", "ultra",
    ];
    let nouns = ["user", "person", "account", "member", "client", "agent"];

    format!(
        "{}{}{}",
        adjectives[rng.gen_range(0..adjectives.len())],
        nouns[rng.gen_range(0..nouns.len())],
        rng.gen_range(1000..9999)
    )
}

/// Generate a random name.
fn generate_random_name() -> String {
    let mut rng = rand::thread_rng();
    let first_names = [
        "Alex", "Jordan", "Taylor", "Morgan", "Casey", "Riley", "Quinn", "Avery",
    ];
    let last_names = [
        "Smith", "Johnson", "Williams", "Brown", "Jones", "Davis", "Miller", "Wilson",
    ];

    format!(
        "{} {}",
        first_names[rng.gen_range(0..first_names.len())],
        last_names[rng.gen_range(0..last_names.len())]
    )
}
