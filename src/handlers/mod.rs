pub mod commit;
pub mod hashes;
pub mod module;
pub mod redis_helper;
pub mod table;
use jct::{Blob, Commit, Tag, Tree};
use actix::Addr;
use actix_redis::{Command, RedisActor};
use actix_web::{delete, get, http::header, http::StatusCode, post, web, HttpResponse, Responder};
use futures::future::{join, join_all};
use redis_async::{resp::RespValue, resp_array};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

const CITY_SCOPE_URL: &str = "https://cityscope.media.mit.edu/CS_cityscopeJS/#/cityioviewer";

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
        .header(header::LOCATION, CITY_SCOPE_URL)
        .finish()
}

#[get("api/dump/")]
pub async fn dump(redis: web::Data<Addr<RedisActor>>) -> impl Responder {
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
    web::Json(dump)
}

#[derive(Debug, Deserialize)]
pub struct Dump {
    blob: Vec<Blob>,
    tree: Vec<Tree>,
    commit: Vec<Commit>,
    tag: Vec<Tag>,
}

#[post("api/restore/")]
pub async fn restore(
    redis: web::Data<Addr<RedisActor>>,
    dump_json: web::Json<Dump>,
) -> impl Responder {
    let dump_json: Dump = dump_json.into_inner();

    let add_blobs = join_all(dump_json.blob.iter().map(|b| redis_helper::add(b, &redis)));

    let add_trees = join_all(dump_json.tree.iter().map(|t| redis_helper::add(t, &redis)));

    let add_commits = join_all(
        dump_json
            .commit
            .iter()
            .map(|c| redis_helper::add(c, &redis)),
    );

    let add_tags = join_all(dump_json.tag.iter().map(|t| redis_helper::add(t, &redis)));

    let ((mut blobs, trees), (commits, tags)) =
        join(join(add_blobs, add_trees), join(add_commits, add_tags)).await;

    blobs.extend(trees);
    blobs.extend(commits);
    blobs.extend(tags);

    match blobs.iter().any(|b| !b) {
        true => "something went wrong",
        false => "ok",
    }
}

#[delete("/api/nuclear/")]
pub async fn nuclear(redis: web::Data<Addr<RedisActor>>) -> impl Responder {
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
        true => "ok".with_status(StatusCode::OK),
        false => "ok".with_status(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn json_error(mes: &str) -> web::Json<Value> {
    web::Json(serde_json::json!({"status":"error", "mes":mes.to_string()}))
}

