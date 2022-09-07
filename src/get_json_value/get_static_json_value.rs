extern crate serde_json;

use serde_json::Value as JsonValue;

// use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

// #[derive(Deserialize)]
// struct Settings {
//     connection_string: String,
// }

pub fn get_static_json_value((&sectionName): (str)) {
    let mut file = File::open("static/setting.json").expect("File not found.");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("Error while reading file");
    let settings: Result<JsonValue, serde_json::Error> = serde_json::from_str(&buffer);
    if settings.is_ok() {
        let p: JsonValue = settings.expect(sectionName);
        return &p;
    }
}
