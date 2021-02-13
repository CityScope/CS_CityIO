use bs58::encode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt::Debug;
use crate::model::Settable;
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub hash: String,
    pub table_name: String,
    pub ts: String,
    pub data: serde_json::Value, 
}

impl Settable for Table {
    fn domain_prefix() -> String {
        String::from("table")
    }

    fn id(&self) -> String {
        self.hash.to_string()
    }

    fn list_item(&self) -> String {
        serde_json::to_string(&vec![&self.hash, &self.table_name]).expect("should be Serializable")
    }
}

impl Table {
}

#[derive(Deserialize, Debug)]
pub struct PartialTable {
    table_name: String,
    data: serde_json::Value,
}

impl From<PartialTable> for Table {
    fn from(partial: PartialTable) -> Self {
        let ts = Utc::now().to_string();
        let hash = "hash".to_string();
        
        Self {
            hash,
            table_name: partial.table_name,
            data: partial.data,
            ts
        }
    }
}
