extern crate cs_cityio_backend;
extern crate diesel;

use base64::encode;
use cs_cityio_backend::{connect, create_user};

fn main() {
    let connection = connect();

    // need base variable
    //
    let credentials = "kermit:CityScience";
    let base = encode(credentials);

    let user = create_user(&connection, &base, false);

    println!("\n saved user {} with id {}", &user.username, &user.id);
}
