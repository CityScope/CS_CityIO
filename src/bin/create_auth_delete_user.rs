extern crate cs_cityio_backend;
extern crate diesel;
use base64::encode;
use cs_cityio_backend::{auth_user, connect, create_user, delete_users};
fn main() {
    let connection = connect();

    // you need to come up with a good &base

    let user_request = "newuser:password";

    // delete_users(&connection, "newuser");

    let base = encode(user_request);

    let user = create_user(&connection, &base, false);

    println!("\n saved user {} with id {}", &user.username, &user.id);

    let authed_user = auth_user(&connection, "newuser", "password").unwrap();

    assert_eq!(authed_user.id, user.id);

    delete_users(&connection, &authed_user.username);
}
