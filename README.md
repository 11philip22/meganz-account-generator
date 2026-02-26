<p align="center">
  <img src="assets/hero-banner.png" alt="hero pane" width="980">
</p>

<p align="center">
  <a href="https://crates.io/crates/meganz-account-generator"><img src="https://img.shields.io/badge/crates.io-meganz--account--generator-F59E0B?style=for-the-badge&logo=rust&logoColor=white" alt="Crates.io"></a>
  <a href="https://docs.rs/meganz-account-generator"><img src="https://img.shields.io/badge/docs.rs-meganz--account--generator-3B82F6?style=for-the-badge&logo=readthedocs&logoColor=white" alt="Documentation"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-8B5CF6?style=for-the-badge" alt="MIT License"></a>
  <a href="https://github.com/woldp001/meganz-account-generator/pulls"><img src="https://img.shields.io/badge/PRs-Welcome-22C55E?style=for-the-badge" alt="PRs Welcome"></a>
</p>

<p align="center">
  <a href="#features">Features</a> · <a href="#usage-as-library">Usage as Library</a> · <a href="#running-the-cli-example">Running the CLI Example</a> · <a href="#cli-options">CLI Options</a> · <a href="#documentation">Documentation</a> · <a href="#contributing">Contributing</a> · <a href="#support">Support</a> · <a href="#license">License</a>
</p>

---

## Features

- **Automated Email**: Uses GuerrillaMail to generate temporary email addresses
- **Auto-Verification**: Automatically polls for the MEGA confirmation email and extracts the verification link
- **Account Creation**: Handles the full registration and verification handshake
- **Library & CLI**: Use as a Rust library or run the included CLI example

## Usage as Library

Add to your `Cargo.toml`:

```toml
[dependencies]
meganz-account-generator = "0.4.3"
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

## Documentation

For detailed API documentation, visit [docs.rs/meganz-account-generator](https://docs.rs/meganz-account-generator).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/cool-feature`)
3. Commit your changes (`git commit -m 'Add some cool feature'`)
4. Push to the branch (`git push origin feature/cool-feature`)
5. Open a Pull Request


## Support

If this crate saves you time or helps your work, support is appreciated:

[![Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/11philip22)

## License

This project is licensed under the MIT License; see the [license](https://opensource.org/licenses/MIT) for details.
