use crate::redis_helper;
use actix::Addr;
use actix_redis::RedisActor;
use actix_web::web;
use serde::de::DeserializeOwned;
use serde::Serialize;

use jct::{Blob, Commit, Settable, Tag, Tree};

pub type Module = Blob;
pub type Hashes = Tree;
pub type Table = Tag;

type RActor = web::Data<Addr<RedisActor>>;

pub async fn get_redis<T>(id: &str, prefix: &str, redis: &RActor) -> Option<T>
where
    T: DeserializeOwned,
{
    match redis_helper::get_slice(id, prefix, &redis).await {
        None => None,
        Some(v) => {
            let object = serde_json::from_slice(&v).expect("object has Deserialize");
            Some(object)
        }
    }
}

pub async fn set_redis<T>(obj: T, redis: &RActor) -> bool
where
    T: Settable,
{
    redis_helper::add(&obj, &redis).await
}

#[derive(Debug, Serialize)]
pub struct Meta {
    id: String,
    hash: String,
    hashes: Tree,
    timestamp: String,
}

impl Meta{
    pub fn new(id:&str, hash: &str, hashes: &Tree, timestamp: &str) -> Self {
        Self {
            id: id.to_string(),
            hash: hash.to_string(),
            hashes: hashes.to_owned(),
            timestamp: timestamp.to_owned()
        }
    }
}
