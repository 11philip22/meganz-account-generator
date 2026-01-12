//! MEGA.nz Account Generator CLI
//!
//! Usage:
//!   meganz-account-generator --password <PASSWORD> [--name <NAME>] [--count <N>] [--output <FILE>]

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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("🚀 MEGA.nz Account Generator");
    println!("{}", "=".repeat(40));

    let generator = match AccountGenerator::new().await {
        Ok(g) => g,
        Err(e) => {
            eprintln!("❌ Failed to initialize: {}", e);
            std::process::exit(1);
        }
    };

    let mut successful = 0;

    for i in 1..=args.count {
        if args.count > 1 {
            println!("\n📋 Generating account {}/{}...", i, args.count);
        }

        match generator.generate(&args.password, args.name.as_deref()).await {
            Ok(account) => {
                successful += 1;
                println!("\n{}", "=".repeat(40));
                println!("✅ Account created successfully!");
                println!("{}", account);
                println!("{}", "=".repeat(40));

                // Save to file if specified
                if let Some(ref output_path) = args.output {
                    if let Err(e) = save_to_file(output_path, &account) {
                        eprintln!("⚠️  Failed to save to file: {}", e);
                    } else {
                        println!("💾 Saved to {}", output_path);
                    }
                }
            }
            Err(e) => {
                eprintln!("\n❌ Failed to generate account: {}", e);
            }
        }

        // Add delay between accounts to avoid rate limiting
        if i < args.count {
            println!("\n⏳ Waiting 30 seconds before next account...");
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }

    println!("\n{}", "=".repeat(40));
    println!("📊 Summary: {}/{} accounts created successfully", successful, args.count);
}

fn save_to_file(path: &str, account: &meganz_account_generator::GeneratedAccount) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "---")?;
    writeln!(file, "Email: {}", account.email)?;
    writeln!(file, "Password: {}", account.password)?;
    writeln!(file, "Name: {}", account.name)?;
    writeln!(file)?;

    Ok(())
}
