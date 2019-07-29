extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect, read_heads, read_table_hash, read_latest_tables};
use cs_cityio_backend::models::User;
use crate::diesel::prelude::*;


fn main() {
    let connection = connect();
    let tables = read_latest_tables(&connection);
    println!("{:?}", tables.unwrap().len());
}
