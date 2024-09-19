#[cfg(test)]
mod tests;
mod utilities;

use std::collections::HashMap;

use arboard::Clipboard;
use clap::Parser;
use inquire::Select;

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

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let client = TMail::parse();

    match client {
        TMail::Generate => match utilities::create_account() {
            Ok(a) => {
                println!("{a} copied to clipboard!");
                clipboard.set_text(a).unwrap();
            }
            Err(e) => println!("Unable to generate new address: {e}"),
        },
        TMail::Delete => match utilities::delete_account() {
            Ok(o) => {
                if o {
                    println!("Account deleted")
                } else {
                    println!("Unable to delete account")
                }
            }
            Err(e) => println!("Unable to delete account: {e}"),
        },
        TMail::Fetch => match utilities::retrieve_messages() {
            Ok(m) => {
                if m.is_empty() {
                    return println!("There are no messages");
                }

                let mut kv = HashMap::new();
                for e in m {
                    kv.insert(
                        e["subject"].as_str().unwrap().to_owned(),
                        e["id"].as_str().unwrap().to_owned(),
                    );
                }

                let c = Select::new(
                    "Select a message",
                    kv.clone().into_iter().map(|x| x.1).collect(),
                )
                .prompt();

                let id: &String = match c {
                    Ok(k) => kv.get(&k).unwrap(),
                    Err(e) => {
                        return println!("Unable to open message: {e}");
                    }
                };

                // if this fails, god help
                let res = match ureq::get(&format!("https://api.mail.tm/messages/{}", *id)).call() {
                    Ok(r) => serde_json::from_str::<serde_json::Value>(&r.into_string().unwrap())
                        .unwrap(),
                    Err(e) => {
                        return println!("Unable to open message: {e}");
                    }
                };
            }
            Err(e) => println!("Unable to retrieve messages: {e}"),
        },
        TMail::Me => match utilities::get_details() {
            Ok(a) => {
                println!("{a} copied to clipboard!");
                clipboard.set_text(a).unwrap();
            }
            Err(e) => println!("Does not exist"),
        },
    }
}
