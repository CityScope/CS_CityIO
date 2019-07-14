#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use std::str;
use chrono::prelude::*;
use serde_json::Value;

use sha256::sha256::{hash, format_hash};

use crate::models::{NewTable, NewUser, Table, User};

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("env var DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_table<'a>(con: &PgConnection, title:&'a str, data: &'a Value) -> Table {
    use schema::tables;

    let new_table = NewTable {
        title: title,
        data: data,
    };

    diesel::insert_into(tables::table)
        .values(&new_table)
        .get_result(con)
        .expect("Error creating new table")
}

pub fn create_user<'a>(con: &PgConnection, base64:&'a str) -> User {
    use schema::users;

    let now: DateTime<Utc> = Utc::now();

    let base = base64.to_owned();

    let b = base64::decode(base64).expect("Error decoding base64");
    let comb = str::from_utf8(&b).expect("Error converting to UTF-8").to_string(); // "username:password"
    let name_pass: Vec<&str> = comb.split(":").collect();
    let username = &name_pass[0];

    let new_base = format!("{} {:?}", &base, &now);

    println!("{}", &new_base);

    let hash = format_hash(&hash(&new_base));

    let new_user = NewUser {
        username: &username,
        ts: &now,
        hash: &hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(con)
        .expect("Error creating new user")
}
