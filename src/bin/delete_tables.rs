extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect, delete_table, delete_head};
use cs_cityio_backend::models::Table;
use crate::diesel::prelude::*;


fn main() {
    use cs_cityio_backend::schema::tables::dsl::*;

    let connection = connect();

    let results = tables.limit(5).load::<Table>(&connection)
        .expect("Error loading tables");

    println!("Displaying {} tables", results.len());
    for table  in results {
        println!("{}", table.table_name);
        println!("---------------\n");
        // println!("{:?}", table.data);

        delete_table(&connection, &table.hash);
        delete_head(&connection, &table.table_name);

        println!("\n");
    }
}
