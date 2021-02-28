pub mod module;
pub mod commit;
pub mod table;
pub mod hashes;
pub mod redis_helper;

use jct::{Blob, Commit, Tag, Tree};

use actix::Addr;
use actix_redis::{Command, RedisActor};
use actix_web::{web, Error as AWError, HttpResponse};
use futures::future::{join, join_all};
use redis_async::{resp::RespValue, resp_array};
use serde_json::Value;
use serde::Deserialize;
use std::collections::BTreeMap;

pub async fn dump(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, AWError> {
    let mut dump: BTreeMap<String, Vec<Value>> = BTreeMap::new();

    let elements = vec!["blob", "tree", "commit", "tag"];

    for element in elements {
        match redis
            .send(Command(resp_array!["smembers", format!("{}s", &element)]))
            .await
        {
            Ok(Ok(RespValue::Array(array))) => {
                let mut list: Vec<String> = Vec::new();
                for item in array {
                    if let RespValue::BulkString(x) = item {
                        let item_string = String::from_utf8(x).expect("ids are utf8");
                        list.push(item_string);
                    }
                }

                let get_list = list
                    .iter()
                    .map(|d| redis_helper::get_slice(&d, element, &redis));

                let mut items: Vec<Value> = Vec::new();

                for item in join_all(get_list).await {
                    if let Some(v) = item {
                        let value: Value =
                            serde_json::from_slice(&v).expect("this should be Deserializable");
                        items.push(value);
                    }
                }

                dump.insert(element.to_string(), items);
            }
            _ => (),
        };
    }
    Ok(HttpResponse::Ok().json(dump))
}

#[derive(Debug, Deserialize)]
pub struct Dump {
    blob: Vec<Blob>,
    tree: Vec<Tree>,
    commit: Vec<Commit>,
    tag: Vec<Tag>,
}

pub async fn restore(
    redis: web::Data<Addr<RedisActor>>,
    dump: web::Json<Dump>,
) -> Result<HttpResponse, AWError> {
    let dump = dump.into_inner();

    let add_blobs = join_all(dump.blob.iter().map(|b| redis_helper::add(b, &redis)));

    let add_trees = join_all(dump.tree.iter().map(|t| redis_helper::add(t, &redis)));

    let add_commits = join_all(dump.commit.iter().map(|c| redis_helper::add(c, &redis)));

    let add_tags = join_all(dump.tag.iter().map(|t| redis_helper::add(t, &redis)));

    let ((mut blobs, trees), (commits, tags)) =
        join(join(add_blobs, add_trees), join(add_commits, add_tags)).await;

    blobs.extend(trees);
    blobs.extend(commits);
    blobs.extend(tags);

    match blobs.iter().any(|b| !b) {
        true => Ok(HttpResponse::Ok().body("something went wrong")),
        false => Ok(HttpResponse::Ok().body("ok")),
    }
}

pub async fn nuclear(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, AWError> {
    // gets all the data using the list
    let mut domain_list: Vec<String> = Vec::new();

    let elements = vec!["blob", "tree", "commit", "tag"];

    let list_of_lists = elements.iter().map(|p| redis_helper::get_list(&p, &redis));

    // let branch_list = redis_helper::get_list("branch", &redis);

    let get_list = join_all(list_of_lists).await;

    for (i, list) in get_list.iter().enumerate() {
        if let Some(x) = list {
            for item in x {
                let domain = format!("{}:{}", &elements[i], item);
                domain_list.push(domain);
            }
        }
    }

    let delete_all = domain_list
        .iter()
        .map(|domain| redis.send(Command(resp_array!["DEL", domain])));

    let delete_prefix = elements
        .iter()
        .map(|e| redis.send(Command(resp_array!["DEL", format!("{}s", e)])));

    let (delete_all, _) = join(join_all(delete_all), join_all(delete_prefix)).await;

    match delete_all.iter().any(|d| {
        let mut flag = false;
        if let Ok(Ok(RespValue::Integer(x))) = d {
            if x == &1 {
                flag = true;
            }
        }
        flag
    }) {
        true => Ok(HttpResponse::Ok().body("ok")),
        false => Ok(HttpResponse::InternalServerError().finish()),
    }
}
