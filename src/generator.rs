use crate::account::GeneratedAccount;
use crate::errors::{Error, Result};
use crate::random::{generate_random_alias, generate_random_name};
use guerrillamail_client::Client as MailClient;
use megalib::{register, verify_registration};
use regex::Regex;
use std::time::Duration;

/// High-level MEGA account generator.
///
/// `AccountGenerator` orchestrates:
/// - creating a temporary GuerrillaMail address
/// - registering a MEGA account
/// - polling the inbox for a likely confirmation email
/// - extracting the confirmation key from the email body and verifying registration
///
/// Use [`AccountGenerator::new`] for defaults, or [`AccountGenerator::builder`] to customize
/// `proxy`, `timeout`, and `poll_interval`.
///
/// This type is designed to be reused to generate multiple accounts with the same configuration.
pub struct AccountGenerator {
    mail_client: MailClient,
    timeout: Duration,
    poll_interval: Duration,
    proxy: Option<String>,
}

/// Builder for [`AccountGenerator`].
///
/// Defaults:
/// - `timeout`: 300 seconds
/// - `poll_interval`: 5 seconds
/// - `proxy`: disabled
#[derive(Debug, Clone)]
pub struct AccountGeneratorBuilder {
    timeout: Duration,
    poll_interval: Duration,
    proxy: Option<String>,
}

impl AccountGenerator {
    /// Create a builder for configuring an [`AccountGenerator`].
    pub fn builder() -> AccountGeneratorBuilder {
        AccountGeneratorBuilder::default()
    }

    /// Create a new generator with default settings.
    ///
    /// Equivalent to `AccountGenerator::builder().build().await`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Mail`] if the underlying GuerrillaMail client cannot be constructed
    /// (for example due to proxy misconfiguration or transport errors).
    pub async fn new() -> Result<Self> {
        Self::builder().build().await
    }

    /// Generate and confirm a MEGA account using a random display name.
    ///
    /// A random temporary GuerrillaMail alias is always used for the email address.
    ///
    /// After registration succeeds, the generator polls the inbox until it can extract a confirmation key
    /// from a likely MEGA email body and verify the registration.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`Error::Mail`] if GuerrillaMail inbox creation, polling, or message-body fetching fails
    /// - [`Error::Mega`] if MEGA registration or verification fails
    /// - [`Error::EmailTimeout`] if no likely MEGA email is observed before `timeout`
    /// - [`Error::NoConfirmationLink`] if a likely MEGA email is observed before `timeout`, but no confirmation
    ///   key can be extracted from its body
    ///
    /// Polling checks GuerrillaMail every `poll_interval` until `timeout` elapses.
    ///
    /// The timeout is evaluated at the start of each poll iteration. As a result, total wall-clock time may
    /// exceed `timeout` by the duration of an in-flight poll request plus up to one `poll_interval` sleep.
    ///
    /// Cleanup of the temporary inbox is best-effort; deletion errors are ignored after successful confirmation.
    pub async fn generate(&self, password: &str) -> Result<GeneratedAccount> {
        let name = generate_random_name();
        self.generate_inner(password, name).await
    }

    /// Generate and confirm a MEGA account with an explicit display name.
    ///
    /// A random temporary GuerrillaMail alias is always used for the email address.
    ///
    /// # Errors
    ///
    /// Returns the same error variants as [`AccountGenerator::generate`].
    ///
    /// Polling checks GuerrillaMail every `poll_interval` until `timeout` elapses.
    ///
    /// The timeout is evaluated at the start of each poll iteration. As a result, total wall-clock time may
    /// exceed `timeout` by the duration of an in-flight poll request plus up to one `poll_interval` sleep.
    pub async fn generate_with_name(&self, password: &str, name: &str) -> Result<GeneratedAccount> {
        self.generate_inner(password, name.to_string()).await
    }

    async fn generate_inner(
        &self,
        password: &str,
        account_name: String,
    ) -> Result<GeneratedAccount> {
        // Generate random alias
        let alias = generate_random_alias();

        let email = self.mail_client.create_email(&alias).await?;

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
        let mut saw_mega_email = false;

        loop {
            if start.elapsed() >= self.timeout {
                return if saw_mega_email {
                    Err(Error::NoConfirmationLink)
                } else {
                    Err(Error::EmailTimeout)
                };
            }

            let messages = self.mail_client.get_messages(email).await?;

            // Look for MEGA confirmation email
            for msg in &messages {
                if msg.mail_from.contains("mega") || msg.mail_subject.contains("MEGA") {
                    saw_mega_email = true;

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

impl Default for AccountGeneratorBuilder {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(300), // 5 minute timeout
            poll_interval: Duration::from_secs(5),
            proxy: None,
        }
    }
}

impl AccountGeneratorBuilder {
    /// Configure an HTTP proxy URL for MEGA and GuerrillaMail requests.
    ///
    /// The value is forwarded directly to both underlying clients.
    ///
    /// This crate does not validate the URL beyond passing it through to the clients; invalid values are
    /// typically reported as [`Error::Mail`] during [`AccountGeneratorBuilder::build`].
    pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

    /// Configure the maximum time to wait for a confirmation email.
    ///
    /// When this duration elapses, generation fails with:
    /// - [`Error::EmailTimeout`] if no likely MEGA email has been observed
    /// - [`Error::NoConfirmationLink`] if a likely MEGA email was observed, but no confirmation key could be
    ///   extracted from its body
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Configure how often to poll GuerrillaMail for new messages.
    pub fn poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    /// Build an [`AccountGenerator`] with the configured values.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Mail`] if the GuerrillaMail client fails to initialize
    /// (e.g., proxy misconfiguration or network errors).
    pub async fn build(self) -> Result<AccountGenerator> {
        let mail_client = build_mail_client(self.proxy.as_deref()).await?;
        Ok(AccountGenerator {
            mail_client,
            timeout: self.timeout,
            poll_interval: self.poll_interval,
            proxy: self.proxy,
        })
    }
}

async fn build_mail_client(proxy: Option<&str>) -> Result<MailClient> {
    let mut builder = MailClient::builder();
    if let Some(proxy_url) = proxy {
        builder = builder.proxy(proxy_url);
    }
    builder.build().await.map_err(Into::into)
}

/// Extract the confirmation key from a MEGA email body.
fn extract_confirm_key(body: &str) -> Option<String> {
    // MEGA confirmation links look like:
    // https://mega.nz/#confirm<KEY>
    // https://mega.nz/confirm<KEY>

    let valid_patterns = [
        r"https://mega\.nz/#confirm([a-zA-Z0-9_-]+)",
        r"https://mega\.nz/confirm([a-zA-Z0-9_-]+)",
        r#"href="https://mega\.nz/#confirm([^"]+)"#,
        r#"href="https://mega\.nz/confirm([^"]+)"#,
    ];

    for pattern in &valid_patterns {
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
