#[path = "../db_/connection_string.rs"]
mod connection_string;

pub fn test_print() -> &'static str {
    return connection_string::CONNECTION_STRING;
}