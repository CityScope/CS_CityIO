extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::connect;
use cs_cityio_backend::models::User;
use crate::diesel::prelude::*;


fn main() {
    use cs_cityio_backend::schema::users::dsl::*;

    let connection = connect();

    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user  in results {
        println!("{}", user.username);
        println!("---------------\n");
        println!("{}, {}", user.id,  user.hash);
    }
}
