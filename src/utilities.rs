use crate::db_path;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use redb::{Database, TableDefinition};
use std::error::Error;
use ureq::json;

pub(crate) const ACCOUNT: TableDefinition<&str, &str> = TableDefinition::new("account");

pub(crate) fn create_account() -> Result<String, Box<dyn Error>> {
    let database = Database::create(db_path())?;
    let read_transaction = database.begin_read()?;
    if let Ok(table) = read_transaction.open_table(ACCOUNT) {
        if table.get("address").is_ok() {
            return Err(Box::from("account already exists"));
        }
    }

    // calling for domains
    let res = ureq::get("https://api.mail.tm/domains")
        .call()?
        .into_string()?;
    let values: serde_json::Value = serde_json::from_str(&res)?;

    let domain = values["hydra:member"][0]["domain"].as_str().unwrap();
    let address = (Alphanumeric.sample_string(&mut thread_rng(), 8) + "@" + domain).to_lowercase();
    let password = Alphanumeric.sample_string(&mut thread_rng(), 16);

    // creating account
    let res = ureq::post("https://api.mail.tm/accounts")
        .send_json(json!({"address": address, "password": password}))?
        .into_string()?;
    let values: serde_json::Value = serde_json::from_str(&res)?;
    let id = values["id"].as_str().unwrap();

    // retrieving token
    let res = ureq::post("https://api.mail.tm/token")
        .send_json(json!({"address": address, "password": password}))?
        .into_string()?;
    let values: serde_json::Value = serde_json::from_str(&res)?;

    let write_transaction = database.begin_write()?;
    {
        let mut table = write_transaction.open_table(ACCOUNT)?;
        table.insert("address", address.as_str())?;
        table.insert("password", password.as_str())?;
        table.insert("id", id)?;
        table.insert("token", values["token"].as_str().unwrap())?;
    }
    write_transaction.commit()?;

    Ok(address)
}

pub(crate) fn get_details() -> Result<String, Box<dyn Error>> {
    let database = Database::create(db_path())?;
    let read_transaction = database.begin_read()?;
    let table = read_transaction.open_table(ACCOUNT)?;
    let Some(address) = table.get("address")? else {
        return Err(Box::from("address does not exist"));
    };

    Ok(address.value().to_owned())
}

pub(crate) fn delete_account() -> Result<bool, Box<dyn Error>> {
    let database = Database::create(db_path())?;
    let read_transaction = database.begin_read()?;
    let table = read_transaction.open_table(ACCOUNT)?;

    let Some(id) = table.get("id")? else {
        return Err(Box::from("id does not exist"));
    };
    let Some(token) = table.get("token")? else {
        return Err(Box::from("token does not exist"));
    };

    let write_transaction = database.begin_write()?;
    write_transaction.delete_table(ACCOUNT)?;
    write_transaction.commit()?;

    Ok(204
        == ureq::delete(&format!("https://api.mail.tm/accounts/{}", id.value()))
            .set("Authorization", &format!("Bearer {}", token.value()))
            .call()?
            .status())
}

pub(crate) fn retrieve_messages() -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
    let database = Database::create(db_path())?;
    let read_transaction = database.begin_read()?;
    let table = match read_transaction.open_table(ACCOUNT) {
        Ok(t) => t,
        Err(_) => return Err(Box::from("account has not been generated")),
    };

    let res = ureq::get("https://api.mail.tm/messages")
        .set(
            "Authorization",
            &format!("Bearer {}", table.get("token")?.unwrap().value()),
        )
        .call()?
        .into_string()?;
    let values: serde_json::Value = serde_json::from_str(&res)?;
    let emails = values["hydra:member"].as_array().unwrap().to_owned();

    Ok(emails)
}
