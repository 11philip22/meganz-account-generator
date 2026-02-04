use crate::errors::{Error, Result};
use guerrillamail_client::Client as MailClient;
use megalib::{register, verify_registration};
use rand::Rng;
use regex::Regex;
use std::time::Duration;

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
///
/// Use [`AccountGenerator::new`] for default timeouts or
/// [`AccountGenerator::with_timeouts`] for custom polling behavior.
pub struct AccountGenerator {
    mail_client: MailClient,
    timeout: Duration,
    poll_interval: Duration,
    proxy: Option<String>,
}

/// Builder for [`AccountGenerator`].
#[derive(Debug, Clone)]
pub struct AccountGeneratorBuilder {
    timeout: Duration,
    poll_interval: Duration,
    proxy: Option<String>,
}

impl AccountGenerator {
    /// Create a builder for configuring an account generator.
    pub fn builder() -> AccountGeneratorBuilder {
        AccountGeneratorBuilder::default()
    }

    /// Create a new account generator.
    pub async fn new() -> Result<Self> {
        Self::builder().build().await
    }

    /// Create a new account generator with custom timeouts.
    ///
    /// # Arguments
    /// * `timeout` - Maximum time to wait for the confirmation email
    /// * `poll_interval` - How often to poll for new email
    pub async fn with_timeouts(timeout: Duration, poll_interval: Duration) -> Result<Self> {
        Self::builder()
            .timeout(timeout)
            .poll_interval(poll_interval)
            .build()
            .await
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
    pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

    /// Configure the maximum time to wait for a confirmation email.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Configure how often to poll for new confirmation emails.
    pub fn poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    /// Build an [`AccountGenerator`] with the configured values.
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
        "ashen", "bleak", "civic", "cold", "covert", "drift", "echo", "grim", "iron", "kilo",
        "latent", "mute", "neon", "noir", "null", "omni", "pale", "quiet", "shadow", "silent",
        "static", "steel", "thin", "vanta", "acid", "arc", "blight", "brine", "brume", "carbon",
        "choke", "cipher", "cryo", "delta", "dusk", "ember", "feral", "fract", "ghost", "hollow",
        "hush", "ice", "ivory", "jett", "knife", "lunar", "mire", "murk", "mylar", "nadir",
        "night", "obsid", "onyx", "oxide", "plague", "ravel", "razor", "rot", "sable", "scar",
        "shard", "slate", "smoke", "suture", "toxin", "ultra", "umbra", "void", "weld", "wire",
        "wraith", "zero",
    ];
    let nouns = [
        "agent",
        "asset",
        "citizen",
        "client",
        "custodian",
        "drifter",
        "emissary",
        "enrollee",
        "entity",
        "index",
        "inmate",
        "node",
        "observer",
        "operative",
        "proxy",
        "report",
        "sector",
        "signal",
        "subject",
        "witness",
        "archive",
        "backdoor",
        "barrier",
        "census",
        "cipher",
        "command",
        "district",
        "echo",
        "firmware",
        "grid",
        "handler",
        "ledger",
        "lock",
        "mesh",
        "mirror",
        "module",
        "nexus",
        "protocol",
        "relay",
        "rubble",
        "sector",
        "shard",
        "siren",
        "station",
        "terminal",
        "vector",
        "vault",
        "ward",
        "zone",
    ];

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
        "Amina",
        "Chidi",
        "Emeka",
        "Ifunanya",
        "Ifeoma",
        "Kelechi",
        "Ngozi",
        "Obinna",
        "Chinwe",
        "Uche",
        "Zainab",
        "Tunde",
        "Bola",
        "Sade",
        "Ade",
        "Kunle",
        "Amaka",
        "Chiamaka",
        "Chukwuemeka",
        "Oluwaseun",
        "Olamide",
        "Folake",
        "Yetunde",
        "Efe",
        "Nneka",
        "Ugo",
        "Chinonso",
        "Opeyemi",
        "Tope",
        "Ayodele",
        "Zubairu",
        "Hadiza",
        "Akira",
        "Hana",
        "Hiro",
        "Kenji",
        "Mei",
        "Rin",
        "Sora",
        "Yuki",
        "Jin",
        "Minseo",
        "Hyun",
        "Jisoo",
        "Soojin",
        "Daichi",
        "Keiko",
        "Yuna",
        "Kaito",
        "Ren",
        "Hina",
        "Sakura",
        "Takumi",
        "Yuto",
        "Haruka",
        "Aoi",
        "Minho",
        "Jiyoon",
        "Seojun",
        "Eunji",
        "Seoyeon",
        "Joon",
        "Hyejin",
        "Sooyoung",
        "Wei",
        "Jun",
        "Hao",
        "Ying",
        "Lin",
        "Xiu",
        "Bo",
        "Fang",
    ];
    let last_names = [
        "Okafor",
        "Adebayo",
        "Okoye",
        "Olawale",
        "Nwosu",
        "Eze",
        "Ibrahim",
        "Yusuf",
        "Chukwu",
        "Adeyemi",
        "Onyeka",
        "Balogun",
        "Fashola",
        "Umeh",
        "Nnamdi",
        "Sani",
        "Okon",
        "Nwachukwu",
        "Ogunleye",
        "Abiola",
        "Ogunbiyi",
        "Okojie",
        "Ekwueme",
        "Oduro",
        "Uzor",
        "Okpara",
        "Afolabi",
        "Ojo",
        "Adigun",
        "Ibe",
        "Okereke",
        "Nduka",
        "Li",
        "Wang",
        "Zhang",
        "Chen",
        "Liu",
        "Yang",
        "Zhao",
        "Wu",
        "Tanaka",
        "Sato",
        "Suzuki",
        "Watanabe",
        "Takahashi",
        "Yamamoto",
        "Nakamura",
        "Ito",
        "Kobayashi",
        "Kato",
        "Yoshida",
        "Yamada",
        "Sasaki",
        "Mori",
        "Abe",
        "Saito",
        "Kim",
        "Lee",
        "Park",
        "Choi",
        "Jung",
        "Kang",
        "Yoon",
        "Lim",
        "Jeon",
        "Han",
        "Song",
        "Shin",
        "Kwon",
        "Hwang",
        "Jang",
        "Yoo",
    ];

    format!(
        "{} {}",
        first_names[rng.gen_range(0..first_names.len())],
        last_names[rng.gen_range(0..last_names.len())]
    )
}
