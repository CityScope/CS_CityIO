use crate::model::{Settable, Tree};
use bs58::encode;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt::Debug;

// tables are really just tags...

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub hash: String,
    pub prev: String,
    pub table_name: String,
    pub hashes: Tree, // module_name, id
    pub timestamp: String,
}

impl Settable for Table {
    fn domain_prefix() -> String {
        String::from("table")
    }

    fn id(&self) -> String {
        self.hash
    }

}

pub struct TempTable{
    prev: Option<String>,
    table_name: String,
    hashes: Tree
}

impl TempTable{

    pub fn new(
        prev: Option<String>,
        hashes: BTreeMap<String, String>,
        table_name:String
        ) -> Self{

        Self{
            prev,
            hashes,
            table_name
        }

    }

    pub fn root_hash(&self) -> String {
        root_hash(&self.hashes)
    }
}


impl From<TempTable> for Table{
    fn from(tmp: TempTable) -> Self {
        // a BTreeMap of <table_name, hash>
        let hash = tmp.root_hash();
        let timestamp = Utc::now().to_string();
        let prev = match tmp.prev{
            Some(p) => p.to_string(),
            None => "0".to_string()
        };

        Self{
            prev,
            timestamp,
            hash,
            table_name: tmp.table_name.to_string(),
            hashes: tmp.hashes
        }
    }
}

fn root_hash(hashes: &BTreeMap<String, String>) -> String {
    let cat = hashes.iter()
            .map(|(k,v)|format!("{} {}",k,v))
            .collect::<Vec<String>>()
            .join(" ");
    encode(Sha256::digest(cat.as_bytes())).into_string()
}
