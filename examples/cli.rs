//! MEGA.nz Account Generator CLI
//!
//! Usage:
//!   meganz-account-generator --password <PASSWORD> [--name <NAME>] [--count <N>] [--output <FILE>] [--proxy <URL>] [--verbose]

use clap::Parser;
use meganz_account_generator::AccountGenerator;
use std::fs::OpenOptions;
use std::io::Write;

/// MEGA.nz Account Generator - Create accounts using temporary email
#[derive(Parser, Debug)]
#[command(name = "meganz-account-generator")]
#[command(version, about, long_about = None)]
struct Args {
    /// Password for the new account(s)
    #[arg(short, long)]
    password: String,

    /// Name for the account (random if not specified)
    #[arg(short, long)]
    name: Option<String>,

    /// Number of accounts to generate
    #[arg(short, long, default_value = "1")]
    count: u32,

    /// Output file to save credentials (appends to file)
    #[arg(short, long)]
    output: Option<String>,

    /// Proxy URL (e.g., http://127.0.0.1:8080)
    #[arg(long)]
    proxy: Option<String>,

    /// Show detailed per-account output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("🚀 MEGA.nz Account Generator");
    println!("Creating {} account(s)...", args.count);

    let mut builder = AccountGenerator::builder();
    if let Some(proxy_url) = args.proxy {
        builder = builder.proxy(proxy_url);
    }

    let generator = match builder.build().await {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Failed to initialize: {}", e);
            std::process::exit(1);
        }
    };

    let mut successful = 0;

    for i in 1..=args.count {
        if args.verbose {
            println!("\n[{}/{}] Creating account...", i, args.count);
        }

        let result = if let Some(name) = args.name.as_deref() {
            generator.generate_with_name(&args.password, name).await
        } else {
            generator.generate(&args.password).await
        };

        match result {
            Ok(account) => {
                successful += 1;
                if args.verbose {
                    println!("Status: SUCCESS");
                    println!("Email: {}", account.email);
                    println!("Password: {}", account.password);
                    println!("Name: {}", account.name);
                } else {
                    println!("[{}/{}] OK {}", i, args.count, account.email);
                }

                // Save to file if specified
                if let Some(ref output_path) = args.output {
                    if let Err(e) = save_to_file(output_path, &account) {
                        eprintln!("Failed to save to file: {}", e);
                    } else if args.verbose {
                        println!("Saved to {}", output_path);
                    }
                }
            }
            Err(e) => {
                if args.verbose {
                    eprintln!("[{}/{}] Status: FAILED", i, args.count);
                } else {
                    eprintln!("[{}/{}] FAILED {}", i, args.count, e);
                }
                if args.verbose {
                    eprintln!("Reason: {}", e);
                }
            }
        }

        // Add delay between accounts to avoid rate limiting
        if i < args.count {
            if args.verbose {
                println!("\nWaiting 30 seconds before next account...");
            }
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }

    println!("Done: {}/{} successful", successful, args.count);
}

fn save_to_file(
    path: &str,
    account: &meganz_account_generator::GeneratedAccount,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    writeln!(file, "---")?;
    writeln!(file, "Email: {}", account.email)?;
    writeln!(file, "Password: {}", account.password)?;
    writeln!(file, "Name: {}", account.name)?;
    writeln!(file)?;

    Ok(())
}
