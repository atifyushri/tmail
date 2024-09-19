use clap::{command, Command};
extern crate clap;

#[cfg(test)]
mod tests;
mod utilities;

fn main() {
    let matches = command!()
        .subcommands([
            Command::new("generate")
                .about("Create a temporary address")
                .alias("g"),
            Command::new("fetch")
                .about("Fetch messages from inbox")
                .alias("f"),
            Command::new("delete").about("Delete account").alias("d"),
            Command::new("me").about("Retrieve details"),
        ])
        .get_matches();
}
