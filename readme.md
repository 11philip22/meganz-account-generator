# MEGA.nz Account Generator

[![Crates.io](https://img.shields.io/crates/v/meganz-account-generator.svg)](https://crates.io/crates/meganz-account-generator)
[![Documentation](https://docs.rs/meganz-account-generator/badge.svg)](https://docs.rs/meganz-account-generator)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/woldp001/guerrillamail-client-rs/pulls)

<img src="https://upload.wikimedia.org/wikipedia/commons/b/bc/MEGA_logo.png" alt="MEGA Logo" width="300">

Automated account creation for MEGA.nz using temporary email addresses (GuerrillaMail).

## Features

- 📧 **Automated Email**: Uses GuerrillaMail to generate temporary email addresses
- 🤖 **Auto-Verification**: Automatically polls for the MEGA confirmation email and extracts the verification link
- 🔐 **Account Creation**: Handles the full registration and verification handshake
- 📚 **Library & CLI**: Use as a Rust library or run the included CLI example

## Usage as Library

Add to your `Cargo.toml`:

```toml
[dependencies]
meganz-account-generator = "0.3.4"
```

```rust
use meganz_account_generator::AccountGenerator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize generator
    let generator = AccountGenerator::new().await?;

    // specific name
    let account = generator
        .generate_with_name("MySecurePassword123!", "My Name")
        .await?;
    
    // OR random name
    // let account = generator.generate("MySecurePassword123!").await?;

    println!("Created account: {}", account.email);
    println!("Password: {}", account.password);
    
    Ok(())
}
```

## Public API

The library exposes the following public types and methods:

- `Error`: error enum for account generation failures
- `Result<T>`: crate-specific result alias
- `GeneratedAccount`: contains `email`, `password`, and `name`
- `AccountGenerator`:
  - `new() -> Result<AccountGenerator>`: default 5-minute timeout, 5-second poll
  - `with_timeouts(timeout: Duration, poll_interval: Duration) -> Result<AccountGenerator>`
  - `builder() -> AccountGeneratorBuilder`
  - `AccountGeneratorBuilder::proxy(proxy)`
  - `AccountGeneratorBuilder::timeout(timeout)`
  - `AccountGeneratorBuilder::poll_interval(poll_interval)`
  - `AccountGeneratorBuilder::build().await`
  - `generate(password: &str) -> Result<GeneratedAccount>`
  - `generate_with_name(password: &str, name: &str) -> Result<GeneratedAccount>`

## Running the CLI Example

Clone the repository and run the CLI example:

```bash
# Generate one account
cargo run --example cli -- --password "YourStrongPassword!"

# Generate 5 accounts and save to file
cargo run --example cli -- --password "YourStrongPassword!" --count 5 --output accounts.txt

# Specify a custom name
cargo run --example cli -- --password "YourStrongPassword!" --name "Custom User"

# Use an HTTP proxy and verbose output
cargo run --example cli -- --password "YourStrongPassword!" --proxy "http://127.0.0.1:8080" --verbose
```

## CLI Options

```
Options:
  -p, --password <PASSWORD>  Password for the new account(s)
  -n, --name <NAME>          Name for the account (random if not specified)
  -c, --count <COUNT>        Number of accounts to generate [default: 1]
  -o, --output <FILE>        Output file to save credentials (appends to file)
      --proxy <PROXY>        Proxy URL (e.g., http://127.0.0.1:8080)
  -v, --verbose              Show detailed per-account output
  -h, --help                 Print help
  -V, --version              Print version
```

## Contributing

PRs are welcome!  
Please run `cargo fmt` and `cargo clippy` before submitting.

If you’re changing behavior, please document it in the PR.

## Support

If this crate saves you time or helps your work, support is appreciated:

[![Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/11philip22)

## License

This project is licensed under the MIT License; see the [license](license) file for details.
