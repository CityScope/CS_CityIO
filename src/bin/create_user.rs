extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect, create_user};
use cs_cityio_backend::models::Table;
use crate::diesel::prelude::*;

use serde_json::json;

fn main() {
    let connection = connect();

    // you need to come up with a good &base

    let base = "Y2l0eWlvOjVXZ3Q4bmp3bFBrcQ==";

    let user = create_user(&connection, &base, true);

    println!("\n saved user {} with id {}", &user.username, &user.id);
}
