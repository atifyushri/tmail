use std::error::Error;

use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use redb::{Database, TableDefinition};
use ureq::json;

const ACCOUNT: TableDefinition<&str, &str> = TableDefinition::new("account");

pub(crate) fn create_account() -> Result<(), Box<dyn Error>> {
    let database = Database::create("account.redb")?;
    let write_transaction = database.begin_write()?;

    // calling for domains
    let res = ureq::get("https://api.mail.tm/domains")
        .call()?
        .into_string()?;
    let values: serde_json::Value = serde_json::from_str(res.as_str())?;

    let domain = values["hydra:member"][0]["domain"].as_str().unwrap();
    let address = (Alphanumeric.sample_string(&mut thread_rng(), 8) + "@" + domain).to_lowercase();
    let password = Alphanumeric.sample_string(&mut thread_rng(), 16);

    // creating account
    let res = ureq::post("https://api.mail.tm/accounts")
        .send_json(json!({"address": address, "password": password}))?
        .into_string()?;
    let values: serde_json::Value = serde_json::from_str(res.as_str())?;
    let id = values["id"].as_str().unwrap();

    // retrieving token
    let res = ureq::post("https://api.mail.tm/token")
        .send_json(json!({"address": address, "password": password}))?
        .into_string()?;
    let values: serde_json::Value = serde_json::from_str(res.as_str())?;

    {
        let mut table = write_transaction.open_table(ACCOUNT)?;
        table.insert("address", address.as_str())?;
        table.insert("password", password.as_str())?;
        table.insert("id", id)?;
        table.insert("token", values["token"].as_str().unwrap())?;
    }
    write_transaction.commit()?;
    Ok(())
}

pub(crate) fn get_details() -> Result<(), Box<dyn Error>> {
    let database = Database::create("account.redb")?;
    let read_transaction = database.begin_read()?;
    // error handling needed
    let table = read_transaction.open_table(ACCOUNT)?;

    Ok(())
}

pub(crate) fn delete_account() -> Result<bool, Box<dyn Error>> {
    let database = Database::create("account.redb")?;
    let read_transaction = database.begin_read()?;
    let table = read_transaction.open_table(ACCOUNT)?;

    let Some(id) = table.get("id")? else {
        return Err(Box::from("id does not exist"));
    };
    let Some(token) = table.get("token")? else {
        return Err(Box::from("token does not exist"));
    };

    Ok(204
        == ureq::delete(&format!("https://api.mail.tm/account/{}", id.value()))
            .set("Authorization", &format!("Bearer {}", token.value()))
            .call()?
            .status())
}
