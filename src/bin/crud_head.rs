extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect,
                        create_table,
                        delete_table,
                        delete_head,
                        update_head,
                        create_head,
                        };
use sha256::sha256::{hash, format_hash};

use serde_json::json;

fn main() {
    let connection = connect();
    let title = "test_table".to_string();

    let data = json!({"data":"test"});
    let hash_value = format_hash(&hash(&data.to_string())).to_owned();
    let table = create_table(&connection, &hash_value, &title, &data);
    // make head
    let _head = create_head(&connection, &title, &hash_value);

    // make second table
    let data = json!({"data":"test2"});
    let hash_value = format_hash(&hash(&data.to_string())).to_owned();
    let new_table = create_table(&connection, &hash_value, &title, &data);

    // update head
    let _head = update_head(&connection, &title, &hash_value);

    // delete head
    delete_head(&connection, &title);

    // clean up tables
    delete_table(&connection, &new_table.hash);
    delete_table(&connection, &table.hash);
}
