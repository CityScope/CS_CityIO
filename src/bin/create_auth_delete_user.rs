extern crate cs_cityio_backend;
extern crate diesel;

use cs_cityio_backend::{connect, create_user, auth_user, delete_users};
use cs_cityio_backend::models::Table;
use crate::diesel::prelude::*;
use base64::encode;

use serde_json::json;

fn main() {
    let connection = connect();

    // you need to come up with a good &base

    let user_request = "newuser:password";

    // delete_users(&connection, "newuser");

    let base = encode(user_request);

    let user = create_user(&connection, &base, false);

    println!("\n saved user {} with id {}", &user.username, &user.id);

    let authed_user = auth_user(&connection, &base).unwrap();

    assert_eq!(authed_user.id, user.id);

    delete_users(&connection, &authed_user.username);

}
