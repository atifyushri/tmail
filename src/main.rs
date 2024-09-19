// #[cfg(test)]
mod utilities;

use arboard::Clipboard;
use clap::Parser;
use inquire::Select;
use redb::Database;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

#[derive(Parser)]
enum TMail {
    /// Generate a new account
    Generate,
    /// Delete account
    Delete,
    /// Retrieve messages
    Fetch,
    /// Get account details
    Me,
}

fn program_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(format!(".{}", env!("CARGO_PKG_NAME")));
    path
}

fn db_path() -> PathBuf {
    let mut path = program_path();
    path.push("accounts.redb");
    path
}

fn index_path() -> PathBuf {
    let mut path = program_path();
    path.push("index.html");
    path
}

fn main() -> Result<(), Box<dyn Error>> {
    if create_dir(program_path()).is_ok() {}
    let mut clipboard = Clipboard::new()?;
    let client = TMail::parse();

    match client {
        TMail::Generate => {
            let a = utilities::create_account()?;
            println!("> {a} copied to clipboard!");
            clipboard.set_text(a)?;
        }
        TMail::Delete => match utilities::delete_account()? {
            true => println!("> Account deleted"),
            false => println!("Unable to delete account"),
        },
        TMail::Me => {
            let a = utilities::get_details()?;
            println!("> {a} copied to clipboard!");
            clipboard.set_text(a)?;
        }
        TMail::Fetch => {
            let m = utilities::retrieve_messages()?;
            if m.is_empty() {
                return Err(Box::from("inbox is empty"));
            }

            let mut kv = HashMap::new();
            for e in m {
                kv.insert(
                    e["subject"].as_str().unwrap().to_owned()
                        + " - "
                        + e["from"]["address"].as_str().unwrap(),
                    e["id"].as_str().unwrap().to_owned(),
                );
            }

            let c = Select::new(
                "Select a message",
                kv.clone().into_iter().map(|x| format!("{}", x.0)).collect(),
            )
            .prompt();

            let database = Database::create(db_path())?;
            let read_transaction = database.begin_read()?;
            let table = read_transaction.open_table(utilities::ACCOUNT)?;

            let res = ureq::get(&format!(
                "https://api.mail.tm/messages/{}",
                kv.get(&c?).unwrap()
            ))
            .set(
                "Authorization",
                &format!("Bearer {}", table.get("token")?.unwrap().value()),
            )
            .call()?
            .into_string()?;

            let values = serde_json::from_str::<serde_json::Value>(&res)?;
            let html = values["html"].as_array().unwrap();
            if html.len() == 0 {
                return Err(Box::from("No HTML content"));
            }

            let mut file = File::create(index_path())?;
            file.write_all(b"")?;
            file.write_all(&html[0].as_str().unwrap().as_bytes())?;
            open::that(index_path())?;
        }
    };

    Ok(())
}
