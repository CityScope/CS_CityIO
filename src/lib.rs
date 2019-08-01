#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use chrono::prelude::*;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel::{PgConnection, result::Error, result::DatabaseErrorKind};
use dotenv::dotenv;
use serde_json::Value;
use std::env;
use std::str;

use sha256::sha256::{format_hash, hash};
use crate::models::{Head, NewHead, NewTable, NewUser, Table, User};
use base64::{decode, encode};

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
) -> Result<Table, Error> {
    use schema::tables;

    let new_table = NewTable {
        hash: hash_value,
        table_name: table_name,
        data: data,
    };

    diesel::insert_into(tables::table)
        .values(&new_table)
        .get_result(con)
}

pub fn send_table<'a>(
    con: &PgConnection,
    hash_value: &'a str,
    table_name: &'a str,
    data: &'a Value,
) -> Result<(), Error> {

    let table = match update_table(&con, &hash_value, &data) {
        Ok(t) => t,
        Err(e) => {
            match create_table(&con, &hash_value, &table_name, &data) {
                Ok(t) => t,
                Err(e) => return Err(e),
            }
        },
    };

    println!("sent table: {}", &table.table_name);

    let head = match update_head(&con, &table.table_name, &table.hash) {
        Ok(h) => h,
        Err(e) => {
            match create_head(&con, &table.table_name, &table.hash) {
                Ok(h) => h,
                Err(e) => return Err(e)
            }
        }
    };

    println!("updated head: {}", &table.table_name);

    Ok(())
}

pub fn read_users(con: &PgConnection) -> Result<Vec<User>, Error> {
    use schema::users::dsl::*;
    users.load::<User>(con)
}

pub fn update_table<'a> (
     con: &PgConnection,
    hash_value: &'a str,
    new_data: &'a Value,
) -> Result<Table, Error>{
// ){
    use schema::tables::dsl::*;
    diesel::update(tables.find(&hash_value))
        .set(data.eq(&new_data))
        .get_result::<Table>(con)
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

pub fn create_head<'a>(con: &PgConnection, name: &'a str, hash_value: &'a str)
-> Result<Head, Error> {
    use schema::heads;

    let new_head = NewHead{
        table_name: name,
        table_hash: hash_value,
    };

    diesel::insert_into(heads::table)
        .values(&new_head)
        .get_result(con)
}

pub fn update_head<'a>(con: &PgConnection, name: &'a str, hash_value: &'a str) -> Result<Head, Error> {
    use schema::heads::dsl::{heads, table_hash};

    diesel::update(heads.find(&name))
        .set(table_hash.eq(hash_value))
        .get_result::<Head>(con)
}

pub fn delete_head<'a>(con: &PgConnection, name: &'a str) {
    use schema::heads::dsl::{heads};

    let result = diesel::delete(heads.find(&name))
        .execute(con)
        .expect("Error deleting head");

    println!("Deleted {:?}", result);
}

pub fn create_user<'a>(con: &PgConnection, base64: &'a str, is_super: bool) -> User {
    use schema::users;

    let now: DateTime<Utc> = Utc::now();

    let base = base64.to_owned();

    let b = decode(base64).expect("Error decoding base64");
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
        is_super: is_super,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(con)
        .expect("Error creating new user")
}

pub fn delete_user(con: &PgConnection, id: i32) {
    use schema::users::dsl::users;

    let result = diesel::delete(users.find(id))
        .execute(con)
        .expect("Error deleting users");

    println!("Deleted {:?}", result);
}

pub fn delete_users<'a>(con: &PgConnection, name:&'a str) {
    use schema::users::dsl::{users, id, username};

    let us = users.filter(username.eq(name)).load::<User>(con).unwrap();

    for u in us {
        diesel::delete(users.find(u.id)).execute(con).unwrap();
    }
}

pub fn auth_user<'a>(con: &PgConnection, name: &'a str, pw: &'a str) -> Option<User> {
    use schema::users::dsl::{users, username};

    let usrs = match users.filter(username.eq(name)).load::<User>(con){
        Ok(u) => u,
        Err(e) => {
            println!("Error getting users");
            return None
        }
    };

    let base64 = encode(&format!("{}:{}", name, pw));
    println!("{}", &base64);

    for u in usrs {
        let base = format!("{} {:?}", base64, u.ts);
        let base_hashed = format_hash(&hash(&base));
        if base_hashed == u.hash {
            return Some(u);
        }
    }

    None
}

pub fn read_heads(con: &PgConnection) -> Result<Vec<Head>, Error> {
    use schema::heads::dsl::*;
    heads.load::<Head>(con)
}

pub fn read_table_hash(con: &PgConnection, hash_value: &str) -> Result<Table, Error> {
    use schema::tables::dsl::*;
    tables.find(hash_value)
        .get_result(con)
}

pub fn read_latest_tables(con: &PgConnection) -> Option<Vec<Table>>{
    let heads = match read_heads(con) {
        Ok(hs) => hs,
        Err(e) => {
            println!("Error, reading heads");
            return None
        }
    };
    let mut tables: Vec<Table> = Vec::new();
    for h in heads {
        let table = match read_table_hash(con, &h.table_hash) {
            Ok(t) => t,
            Err(e) => continue
        };
        tables.push(table);
    }

    if tables.len() != 0 {
        Some(tables)
    } else {
        None // :(
    }
}
