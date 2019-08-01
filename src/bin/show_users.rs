extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect, read_users};

fn main() {
    let connection = connect();

    let users = match read_users(&connection) {
        Ok(u) => u,
        Err(e) => return println!("Error: {}", e),
    };

    println!("Displaying {} users", users.len());
    for user in users {
        println!("{}", user.username);
        println!("---------------\n");
        println!("{}, {}, is_super: {}", user.id, user.hash, user.is_super);
    }
}
