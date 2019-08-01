extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect, read_latest_tables};

fn main() {
    let connection = connect();
    let tables = read_latest_tables(&connection);
    println!("{:?}", tables.unwrap().len());
}
