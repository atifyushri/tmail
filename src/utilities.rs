use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use redb::{Database, TableDefinition};
use ureq::json;

const ACCOUNT: TableDefinition<&str, &str> = TableDefinition::new("account");

pub(crate) fn create_account() {
    let database = Database::create("account.redb").expect("Unable to create database");
    let write_transaction = database
        .begin_write()
        .expect("Unable to begin writing to database");

    let res = ureq::get("https://api.mail.tm/domains")
        .call()
        .expect("Unable to create account")
        .into_string()
        .expect("Unable to convert to String");

    let values: serde_json::Value =
        serde_json::from_str(res.as_str()).expect("Unable to parse response");

    let domain = values["hydra:member"][0]["domain"].as_str().unwrap();
    let address = (Alphanumeric.sample_string(&mut thread_rng(), 8) + "@" + domain).to_lowercase();
    let password = Alphanumeric.sample_string(&mut thread_rng(), 16);

    let res = ureq::post("https://api.mail.tm/accounts")
        .send_json(json!({
            "address": address,
            "password": password
        }))
        .expect("Unable to create account")
        .into_string()
        .expect("Unable to convert to String");

    let values: serde_json::Value =
        serde_json::from_str(res.as_str()).expect("Unable to parse response");

    let id = values["id"].as_str().unwrap();

    let res = ureq::post("https://api.mail.tm/token")
        .send_json(json!({
            "address": address,
            "password": password
        }))
        .expect("Unable to create token")
        .into_string()
        .expect("Unable to convert to String");
    let values: serde_json::Value =
        serde_json::from_str(res.as_str()).expect("Unable to parse response");

    {
        let mut table = write_transaction
            .open_table(ACCOUNT)
            .expect("Unable to open table");
        table
            .insert("address", address.as_str())
            .expect("Unable to insert address");
        table
            .insert("password", password.as_str())
            .expect("Unable to insert password");
        table.insert("id", id).expect("Unable to insert id");
        table
            .insert("token", values["token"].as_str().unwrap())
            .expect("Unable to insert id");
    }
    write_transaction
        .commit()
        .expect("Transaction unable to be completed");

    let read_transaction = database.begin_read().expect("Unable to start read");
    let table = read_transaction
        .open_table(ACCOUNT)
        .expect("Unable to open table");
}

pub(crate) fn get_details() {
    let database = Database::create("account.redb").expect("Unable to create database");
    let read_transaction = database.begin_read().expect("Unable to start read");
    // error handling
    let table = read_transaction
        .open_table(ACCOUNT)
        .expect("Unable to open table");
}

pub(crate) fn delete_account() -> bool {
    let database = Database::create("account.redb").expect("Unable to create database");
    let read_transaction = database.begin_read().expect("Unable to start read");
    // error handling needed
    let table = read_transaction
        .open_table(ACCOUNT)
        .expect("Unable to open table");

    204 == ureq::delete(
        format!(
            "https://api.mail.tm/accounts/{}",
            table
                .get("id")
                .expect("unable to retrieve id")
                .expect("id does not exist")
                .value()
        )
        .as_str(),
    )
    .set(
        "Authorization",
        format!(
            "Bearer {}",
            table
                .get("token")
                .expect("unable to retrieve token")
                .expect("token does not exist")
                .value()
        )
        .as_str(),
    )
    .call()
    .expect("unable to delete account")
    .status()
}
