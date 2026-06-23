<div align="center">
  <h1>MEGA.nz Account Generator</h1>
  <p><strong>Automated MEGA.nz account registration and confirmation for Rust.</strong></p>

  <p>
    <a href="https://crates.io/crates/meganz-account-generator"><img src="https://img.shields.io/crates/v/meganz-account-generator?style=for-the-badge&logo=rust&color=F59E0B" alt="Crates.io"></a>
    <a href="https://docs.rs/meganz-account-generator"><img src="https://img.shields.io/docsrs/meganz-account-generator?style=for-the-badge&logo=readthedocs&color=3B82F6" alt="docs.rs"></a>
    <a href="https://crates.io/crates/meganz-account-generator"><img src="https://img.shields.io/crates/l/meganz-account-generator?style=for-the-badge&color=10B981" alt="License"></a>
    <a href="https://github.com/11philip22/meganz-account-generator"><img src="https://img.shields.io/badge/Rust-2024-8B5CF6?style=for-the-badge&logo=rust&logoColor=white" alt="Rust 2024"></a>
  </p>

  <p>
    <a href="#features">Features</a>
    &middot;
    <a href="#install">Install</a>
    &middot;
    <a href="#library-usage">Library Usage</a>
    &middot;
    <a href="#cli-example">CLI Example</a>
    &middot;
    <a href="#configuration">Configuration</a>
  </p>
</div>

---

`meganz-account-generator` creates a temporary GuerrillaMail inbox, starts a MEGA.nz registration, polls for the confirmation email, extracts the confirmation key, verifies the account, and returns the generated credentials.

> [!IMPORTANT]
> This crate depends on external MEGA.nz and GuerrillaMail behavior. Use account automation only where it is permitted, and expect failures if upstream APIs, email delivery, or rate limits change.

## Features

- End-to-end account registration and email confirmation.
- Random temporary email aliases and random display names.
- Optional explicit account display names.
- Proxy support for both MEGA.nz and GuerrillaMail requests.
- Reusable async generator with configurable timeout and polling interval.
- CLI example for one-off or repeated account creation.

## Install

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
meganz-account-generator = "0.4.4"
```

The crate uses Tokio, so your application needs an async runtime.

## Library Usage

```rust
use std::time::Duration;
use meganz_account_generator::AccountGenerator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generator = AccountGenerator::builder()
        .timeout(Duration::from_secs(180))
        .poll_interval(Duration::from_secs(3))
        // .proxy("http://127.0.0.1:8080")
        .build()
        .await?;

    let account = generator
        .generate_with_name("S3cure-Password!", "Automation Bot")
        .await?;

    println!("Created account: {}", account.email);
    println!("Name: {}", account.name);

    Ok(())
}
```

For a random display name, call `generate(password)` instead of `generate_with_name(password, name)`.

## CLI Example

Run the included example from a checkout:

```bash
cargo run --example cli -- --password "YourStrongPassword!"
cargo run --example cli -- --password "YourStrongPassword!" --name "Custom User"
cargo run --example cli -- --password "YourStrongPassword!" --count 5 --output accounts.txt
cargo run --example cli -- --password "YourStrongPassword!" --proxy "http://127.0.0.1:8080" --verbose
```

CLI options:

| Option | Description |
| --- | --- |
| `-p, --password <PASSWORD>` | Password for generated accounts. |
| `-n, --name <NAME>` | Account display name. Random when omitted. |
| `-c, --count <COUNT>` | Number of accounts to create. Defaults to `1`. |
| `-o, --output <FILE>` | Append generated credentials to a file. |
| `--proxy <PROXY>` | Proxy URL, such as `http://127.0.0.1:8080`. |
| `-v, --verbose` | Print detailed per-account output. |

## Configuration

`AccountGenerator::new().await` uses the default settings. Use `AccountGenerator::builder()` when you need to customize runtime behavior.

| Setting | Default | Purpose |
| --- | --- | --- |
| `timeout` | `300s` | Maximum time to wait for a likely MEGA.nz confirmation email. |
| `poll_interval` | `5s` | Delay between GuerrillaMail inbox checks. |
| `proxy` | Disabled | Optional proxy forwarded to both underlying clients. |

Generation returns `GeneratedAccount` only after registration is confirmed. Failures are reported as `Error::Mail`, `Error::Mega`, `Error::EmailTimeout`, or `Error::NoConfirmationLink`.

## Documentation

API documentation is available on [docs.rs/meganz-account-generator](https://docs.rs/meganz-account-generator).
