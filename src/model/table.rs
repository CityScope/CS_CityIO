use crate::{model::{Settable, Module}, redis_helper::redis_get_slice};
use actix_web::web;
use actix::Addr;
use actix_redis::RedisActor;
use bs58::encode;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt::Debug;
use serde_json::Value;
use futures::future::join_all;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub hash: String,
    pub prev: String,
    pub table_name: String,
    pub hashes: BTreeMap<String, String>, // module_name, id
    pub timestamp: String,
}

impl Table {
    pub async fn compile(&self, redis: &web::Data<Addr<RedisActor>>) -> BTreeMap<String, Value>{

        let mut result: BTreeMap<String, Value> = BTreeMap::new();

        let modules = self.hashes.iter().map(|(_k,v)|{
            log::debug!("{}", &v);
            redis_get_slice(v, "module", &redis)
        });

        // add self to result
        let modules = join_all(modules).await;

        for m in modules {
            if let Some(x) = m {
                let module: Module = serde_json::from_slice(&x)
                    .expect("module should be Deserializable"); 
                result.insert(module.name(), module.data());
            }
        }

        let table_meta = serde_json::to_value(self).expect("table is deserializable");
        result.insert("meta".to_string(), table_meta);

        result
    }

}

impl Settable for Table {
    fn domain_prefix() -> String {
        String::from("table")
    }

    fn id(&self) -> String {
        self.hash.to_string()
    }

    fn list_item(&self) -> String {
        serde_json::to_string(&vec![&self.hash, &self.table_name])
            .expect("should be Serializable")
    }
}

pub struct TempTable{
    prev: Option<String>,
    table_name: String,
    hashes: BTreeMap<String, String>
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
        let cat = self.hashes.iter()
            .map(|(k,v)|format!("{} {}",k,v))
            .collect::<Vec<String>>()
            .join(" ");

        let cat = vec![self.table_name.to_string(), cat].join(" "); 

        encode(Sha256::digest(cat.as_bytes())).into_string()
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
