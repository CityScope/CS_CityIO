extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect, create_table};
use cs_cityio_backend::models::Table;
use crate::diesel::prelude::*;

use serde_json::json;

fn main() {
    let connection = connect();
    let title = "test_table".to_string();
    let data = json!({"data":"test"});
    let table = create_table(&connection, &title, &data);
    println!("\n saved table {} with id {}", title, table.id);
}
