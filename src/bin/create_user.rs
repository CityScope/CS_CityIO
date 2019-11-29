extern crate cs_cityio_backend;
extern crate diesel;

use base64::encode;
use cs_cityio_backend::{connect, create_user};

fn main() {
    let connection = connect();

    // need base variable
    //
    // let credentials = "kermit:CityScience";

    // let credentials = "mediterranean:gs5eu_pq";
    // let credentials = "dahurian:mvz!@76k";
    // let credentials = "dawyck:kmy4-=uf";
    // let credentials = "pallis:bxr2@h&=";
    // let credentials = "norway:4hv-u9pr";
    let credentials = "briot:hg92w+7q";
    let base = encode(credentials);

    let user = create_user(&connection, &base, false);

    println!("\n saved user {} with id {}", &user.username, &user.id);
}
