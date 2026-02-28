mod auth;
mod client;
mod exporter;
mod models;

use anyhow::Result;
use clap::Parser;
use std::env;
use std::io::{self, Write};
use rpassword::read_password;
use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;

use crate::auth::SkoobAuth;
use crate::client::SkoobClient;
use crate::exporter::SkoobExporter;

#[derive(Parser, Debug)]
#[command(author, version, about = "Export your Skoob bookshelf to JSON and CSV", long_about = None)]
struct Args {
    #[arg(short, long, help = "Your Skoob email")]
    email: Option<String>,

    #[arg(short, long, help = "Your Skoob password")]
    password: Option<String>,

    #[arg(short, long, help = "Manually provide a JWT token")]
    token: Option<String>,

    #[arg(short, long, help = "Manually provide a user ID")]
    user_id: Option<String>,

    #[arg(short, long, default_value = "full_bookshelf", help = "Output file path/basename (without extension)")]
    output: String,

    #[arg(long, help = "Export to JSON format")]
    json: bool,

    #[arg(long, help = "Export to CSV format")]
    csv: bool,
}

fn get_user_id_from_token(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    let payload = parts[1];
    
    // Base64 decoding with padding fix is handled by general_purpose::URL_SAFE_NO_PAD or similar
    // JWT uses URL-safe base64 without padding.
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(payload).ok()?;
    let data: Value = serde_json::from_slice(&decoded).ok()?;
    
    data.get("id").and_then(|id| match id {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = Args::parse();

    // Default to both if neither is specified
    let mut export_json = args.json;
    let mut export_csv = args.csv;
    if !export_json && !export_csv {
        export_json = true;
        export_csv = true;
    }

    // Priority: Argument > Environment Variable
    let mut token = args.token.or_else(|| env::var("SKOOB_AUTH_TOKEN").ok());
    let mut user_id = args.user_id.or_else(|| env::var("SKOOB_USER_ID").ok());
    let email = args.email.or_else(|| env::var("SKOOB_EMAIL").ok());
    let mut password = args.password;

    if token.is_none() {
        if user_id.is_some() {
            println!("[!] Warning: --user-id provided without --token. Proceeding with login...");
        }

        let email = match email {
            Some(e) => e,
            None => {
                eprintln!("[!] Error: Email not provided. Use --email or SKOOB_EMAIL environment variable.");
                eprintln!("[!] Note: If you use Google/Facebook, use --token or set a password in your Skoob account settings.");
                std::process::exit(1);
            }
        };

        if password.is_none() {
            print!("[*] Enter password for {}: ", email);
            io::stdout().flush()?;
            let p = read_password()?;
            if p.is_empty() {
                eprintln!("[!] Error: Password cannot be empty.");
                std::process::exit(1);
            }
            password = Some(p);
        }

        println!("[*] Attempting to login...");
        let auth = SkoobAuth::new()?;
        match auth.signin(&email, &password.unwrap()).await {
            Ok(login_data) => {
                // Flexible extraction from login response
                let t = login_data.token.or_else(|| {
                    login_data.response.as_ref().and_then(|r| r.token.clone())
                });

                let uid = login_data.user.as_ref().map(|u| match &u.id {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    _ => u.id.to_string(),
                }).or_else(|| {
                    login_data.response.as_ref().and_then(|r| r.user.as_ref().map(|u| match &u.id {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => u.id.to_string(),
                    }))
                });

                token = t;
                user_id = uid;

                if user_id.is_none() && token.is_some() {
                    user_id = get_user_id_from_token(token.as_ref().unwrap());
                }

                if token.is_none() || user_id.is_none() {
                    eprintln!("[!] Login response received but token/user_id not found in usual places.");
                    eprintln!("DEBUG: {:?}", login_data);
                    std::process::exit(1);
                }
                
                println!("[+] Login successful! User ID: {}", user_id.as_ref().unwrap());
            }
            Err(e) => {
                eprintln!("[!] Login failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    let token = token.ok_or_else(|| anyhow::anyhow!("Missing token"))?;
    let user_id = user_id.ok_or_else(|| anyhow::anyhow!("Missing user ID"))?;

    let client = SkoobClient::new(token, user_id)?;
    match client.fetch_all_books("book", "all", 1.5).await {
        Ok(books) => {
            if export_json {
                SkoobExporter::to_json(&books, &format!("{}.json", args.output))?;
            }
            if export_csv {
                SkoobExporter.to_csv(&books, &format!("{}.csv", args.output))?;
            }
            println!("\n[DONE] Successfully exported {} books.", books.len());
        }
        Err(e) => {
            eprintln!("[!] Error fetching bookshelf: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
