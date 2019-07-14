use serde_json::Value as JSONValue;
use crate::schema::{tables, users};
use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Queryable)]
pub struct Table {
    pub id: i32,
    pub title: String,
    pub data: JSONValue,
}

#[derive(Insertable)]
#[table_name="tables"]
pub struct NewTable<'a> {
    pub title: &'a str,
    pub data: &'a JSONValue,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash: String,
    pub ts: DateTime<Utc>
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hash: &'a str,
    // hash is the following
    // sha256(base64 + ts)
    pub ts: &'a DateTime<Utc>
}
