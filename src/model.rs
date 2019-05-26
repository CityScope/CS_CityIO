use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};
use serde_json::Value;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use sha256::sha256::{format_hash, hash};
use crate::schema::tables;
use crate::database::{Pool, init_pool};

pub struct CityIOData {
    pub pool: Pool,
    pub tables: HashMap<String, Value>,
}

impl CityIOData {
    pub fn new() -> Self {
        CityIOData {
            pool: init_pool(),
            tables: HashMap::new(),
        }
    }
}

pub type CityIOState = Arc<Mutex<CityIOData>>;

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct Table {
    pub id: String,
    pub ts: NaiveDateTime,
    pub name: String,
    pub data: Value
}

#[derive(Insertable)]
#[table_name="tables"]
pub struct NewTable<'a> {
    pub id: &'a str,
    pub ts: &'a NaiveDateTime,
    pub name: &'a str,
    pub data: &'a Value
}

#[derive(Serialize, Deserialize)]
pub struct TableList(Vec<String>);

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub id: String,
    timestamp: NaiveDateTime,
    apiv: String,
}

impl Meta {
    pub fn new(dump: &str) -> Meta {
        let id = format_hash(&hash(&dump));
        let now = Utc::now().naive_utc();
        let apiv = String::from("2.0");

        Meta {
            id: id,
            timestamp: now,
            apiv: apiv,
        }
    }
}
