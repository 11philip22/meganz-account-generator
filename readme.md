# MEGA.nz Account Generator

[![Crates.io](https://img.shields.io/crates/v/meganz-account-generator.svg)](https://crates.io/crates/meganz-account-generator)
[![Documentation](https://docs.rs/meganz-account-generator/badge.svg)](https://docs.rs/meganz-account-generator)
[![License: GPL v2](https://img.shields.io/badge/License-GPL_v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)

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
meganz-account-generator = "0.1.0"
```

```rust
use meganz_account_generator::AccountGenerator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize generator
    let generator = AccountGenerator::new().await?;

    // specific name
    let account = generator.generate("MySecurePassword123!", Some("My Name")).await?;
    
    // OR random name
    // let account = generator.generate("MySecurePassword123!", None).await?;

    println!("Created account: {}", account.email);
    println!("Password: {}", account.password);
    
    Ok(())
}
```

## Running the CLI Example

Clone the repository and run the CLI example:

```bash
# Generate one account
cargo run --example cli -- --password "YourStrongPassword!"

# Generate 5 accounts and save to file
cargo run --example cli -- --password "YourStrongPassword!" --count 5 --output accounts.txt

# Specify a custom name
cargo run --example cli -- --password "YourStrongPassword!" --name "Custom User"
```

## CLI Options

```
Options:
  -p, --password <PASSWORD>  Password for the new account(s)
  -n, --name <NAME>          Name for the account (random if not specified)
  -c, --count <COUNT>        Number of accounts to generate [default: 1]
  -o, --output <FILE>        Output file to save credentials (appends to file)
  -h, --help                 Print help
```

## License

This project is licensed under the GNU General Public License v2.0 (GPLv2) - see the [license](license) file for details.
