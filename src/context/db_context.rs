#[path = "../get_json_value/get_static_json_value.rs"]
mod json_value;

use mongodb::bson::doc;
use mongodb::Client;

// Requires the MongoDB crate.
// https://crates.io/crates/mongodb

pub fn ConnectionOpen() {
    let client = Client::with_uri_str(json_value::get_static_json_value("DatabaseURI")).await?;
    let result = client
        .database(json_value::get_static_json_value("DatabaseName"))
        .collection::<Document>("customers")
        .find(doc! {}, None)
        .await?;
}
