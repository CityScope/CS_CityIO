#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use chrono::prelude::*;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use serde_json::Value;
use std::env;
use std::str;

use sha256::sha256::{format_hash, hash};

use crate::models::{Head, NewHead, NewTable, NewUser, Table, User};

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("env var DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_table<'a>(
    con: &PgConnection,
    hash_value: &'a str,
    table_name: &'a str,
    data: &'a Value,
) -> Table {
    use schema::tables;

    let new_table = NewTable {
        hash: hash_value,
        table_name: table_name,
        data: data,
    };

    diesel::insert_into(tables::table)
        .values(&new_table)
        .get_result(con)
        .expect("Error creating new table")
}

pub fn delete_table<'a>(con: &PgConnection, hash_value: &'a str){
    use schema::tables::dsl::{tables};

    let result = diesel::delete(tables.find(&hash_value))
        .execute(con)
        .expect("Error deleting table");

    println!("{:?}", result);
   
}

pub fn check_head<'a>(con: &PgConnection, name: &'a str) -> bool {
    use schema::heads::dsl::*;

    select(exists(heads.filter(table_name.eq(&name))))
        .get_result(con)
        .expect("Error checking existance")
}

pub fn create_head<'a>(con: &PgConnection, name: &'a str, hash_value: &'a str) -> Head {
    use schema::heads;

    let new_head = NewHead{
        table_name: name,
        table_hash: hash_value,
    };

    diesel::insert_into(heads::table)
        .values(&new_head)
        .get_result(con)
        .expect("Error creating new head")
}

pub fn update_head<'a>(con: &PgConnection, name: &'a str, hash_value: &'a str) -> Head {
    use schema::heads::dsl::{heads, table_hash};

    let head = diesel::update(heads.find(&name))
        .set(table_hash.eq(hash_value))
        .get_result::<Head>(con)
        .expect("Error updating head hash");

    println!("new hash for {} is {}", head.table_name, head.table_hash);

    head
}

pub fn delete_head<'a>(con: &PgConnection, name: &'a str) {
    use schema::heads::dsl::{heads};

    let result = diesel::delete(heads.find(&name))
        .execute(con)
        .expect("Error deleting head");

    println!("Deleted {:?}", result);
}

pub fn create_user<'a>(con: &PgConnection, base64: &'a str) -> User {
    use schema::users;

    let now: DateTime<Utc> = Utc::now();

    let base = base64.to_owned();

    let b = base64::decode(base64).expect("Error decoding base64");
    let comb = str::from_utf8(&b)
        .expect("Error converting to UTF-8")
        .to_string(); // "username:password"
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
