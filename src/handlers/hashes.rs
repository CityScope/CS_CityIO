use crate::json_error;
use crate::model::Hashes;
use crate::redis_helper;
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{Error as AWError, Responder, delete, get, http::StatusCode, post, web};
use futures::future::join_all;
use jct::{Assign, Settable, Tree};
use redis_async::{resp::RespValue, resp_array};
use serde_json::{json, Value};

#[get("api/hashes/{id}/")]
pub async fn get(redis: web::Data<Addr<RedisActor>>, id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    match redis_helper::get_slice(&id, "tree", &redis).await {
        Some(v) => {
            let r: Value = serde_json::from_slice(&v).unwrap();  
            web::Json(r).with_status(StatusCode::OK)
        },
        None => {
            json_error("hashes not found").with_status(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn add_module(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String, String)>,
) -> Result<impl Responder, AWError> {
    let (hashes_id, blob_name, blob_id) = path.into_inner();

    let mut hashes: Hashes = match redis_helper::get_slice(&hashes_id, "tree", &redis).await {
        Some(t) => serde_json::from_slice(&t).expect("Hashes are Serializable"),
        None => {
            log::debug!("hashes not found");
            let result = json!({"status":"error", "mes":"hashes(tree) not found"});
            return Ok(web::Json(result).with_status(StatusCode::NOT_FOUND));
        }
    };

    // check if blob_id exists;
    let domain = format!("blob:{}", &blob_id);
    let blob_exists = redis.send(Command(resp_array!["EXISTS", &domain])).await?;

    match blob_exists {
        Ok(RespValue::Integer(x)) if x == 1 => (),
        Ok(RespValue::Integer(x)) if x == 0 => {
            log::debug!("module:{} did not exist", &blob_id);
            return Ok(web::Json(json!("module not found")).with_status(StatusCode::NOT_FOUND));
        }
        _ => {
            log::error!("error looking up module:{}", &blob_id);
            return Ok(
                web::Json(json!({"status":"error","mes":"error looking up module"}))
                    .with_status(StatusCode::INTERNAL_SERVER_ERROR),
            );
        }
    }

    // we avoid going to db if there is no change
    let prev_id = &hashes.id();
    hashes.insert(blob_name, blob_id);

    if prev_id == &hashes.id() {
        let result = json!({"status":"ok", "id":&prev_id, "mes":"no change in hash id"});
        return Ok(web::Json(result).with_status(StatusCode::OK));
    }

    match redis_helper::add(&hashes, &redis).await {
        true => {
            // tree will be updated
            let result = json!({"status":"ok", "id":&hashes.id()});
            Ok(web::Json(result).with_status(StatusCode::OK))
        }
        false => {
            log::error!("failed to add updated tree");
            let result = json!({"status":"error", "mes":"failed to add updated tree"});
            Ok(web::Json(result).with_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

pub async fn add_modules(
    redis: web::Data<Addr<RedisActor>>,
    tree_id: web::Path<String>,
    other: web::Json<Tree>,
) -> impl Responder {
    let tree_id = tree_id.into_inner();

    let mut tree: Tree = match redis_helper::get_slice(&tree_id, "tree", &redis).await {
        Some(t) => serde_json::from_slice(&t).expect("Tree is Serializable"),
        None => return json_error("tree not found").with_status(StatusCode::NOT_FOUND),
    };

    let other = other.into_inner();

    let blob_exists = other.values().map(|id| {
        let domain = format!("blob:{}", &id);
        redis.send(Command(resp_array!["EXISTS", &domain]))
    });

    if join_all(blob_exists).await.iter().any(|v| match v {
        Ok(Ok(RespValue::Integer(x))) if x == &1 => false,
        _ => true,
    }) {
        return json_error("blob did not exist").with_status(StatusCode::NOT_FOUND);
    }

    let prev_id = &tree.id();
    tree.assign(&other);

    // we avoid going to db if there is no change
    if prev_id == &tree.id() {
        let result = json!({"status":"ok", "id":&prev_id, "mes":"no change in hash id"});
        return web::Json(result).with_status(StatusCode::OK);
    }

    match redis_helper::add(&tree, &redis).await {
        true => {
            // tree will be updated
            let result = json!({"status":"ok", "id":&tree.id()});
            web::Json(result).with_status(StatusCode::OK)
        }
        false => {
            json_error("failed to add updated tree").with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[delete("api/hashes/{id}/{blob_name}/")]
pub async fn remove_module(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (tree_id, blob_name) = path.into_inner();

    let mut tree: Tree = match redis_helper::get_slice(&tree_id, "tree", &redis).await {
        Some(t) => serde_json::from_slice(&t).expect("Tree is Serializable"),
        None => {
            return json_error("tree not found").with_status(StatusCode::NOT_FOUND);
        }
    };

    match tree.remove(&blob_name) {
        None => {
            let result = json!({"status":"ok", "id":&tree.id(), "mes":"no change in hash id"});
            return web::Json(result).with_status(StatusCode::OK);
        }
        Some(_x) => {
            match redis_helper::add(&tree, &redis).await {
                true => {
                    // tree will be updated
                    let result = json!({"status":"ok", "id":&tree.id()});
                    web::Json(result).with_status(StatusCode::OK)
                }
                false => {
                    log::error!("failed to add updated tree");
                    json_error("failed to add updated tree")
                        .with_status(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    }
}

#[post("api/hashes/")]
pub async fn post(redis: web::Data<Addr<RedisActor>>, payload: web::Json<Tree>) -> impl Responder {
    let b: Tree = payload.into_inner();

    // check if every blob exist
    let blobs_exist = join_all(
        b.values()
            .map(|id| redis.send(Command(resp_array!["EXISTS", &format!("blob:{}", id)]))),
    )
    .await;

    if !blobs_exist
        .iter()
        .map(|result| match result {
            Ok(Ok(RespValue::Integer(x))) if x == &1 => true,
            _ => false,
        })
        .any(|b| b)
    {
        return json_error("blob does not exist").with_status(StatusCode::NO_CONTENT);
    }

    match redis_helper::add(&b, &redis).await {
        true => {
            let result = json!({"status":"ok", "id":&b.id()});
            web::Json(result).with_status(StatusCode::OK)
        }
        false => json_error("failed to update tree").with_status(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
