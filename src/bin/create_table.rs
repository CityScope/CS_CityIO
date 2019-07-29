extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect, create_table};
use cs_cityio_backend::models::Table;
use crate::diesel::prelude::*;
use sha256::sha256::{hash, format_hash};

use serde_json::json;

fn main() {
    let connection = connect();
    let title = "test_table".to_string();
    let data = json!({"data":"test"});
    let hash_value = format_hash(&hash(&data.to_string())).to_owned();
    let table = create_table(&connection, &hash_value, &title, &data);
    println!("\n saved table {} with id {}", title, table.hash);
}
