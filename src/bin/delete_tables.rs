extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect, delete_table, delete_head};
use cs_cityio_backend::models::Head;
use crate::diesel::prelude::*;


fn main() {
    use cs_cityio_backend::schema::heads::dsl::*;

    let connection = connect();

    let results = heads.load::<Head>(&connection)
        .expect("Error loading tables");

    println!("Displaying {} tables", results.len());
    for head in results {
        delete_head(&connection, &head.table_name);
        delete_table(&connection, &head.table_hash);
        println!("\n");
    }
}
