use crate::redis_helper;
use crate::model::Hashes;
use jct::{Settable, Tree, Assign};
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{web, Error as AWError, HttpResponse};
use futures::future::join_all;
use redis_async::{resp::RespValue, resp_array};
use serde_json::json;

pub async fn get(
    redis: web::Data<Addr<RedisActor>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let id = id.into_inner();
    redis_helper::get(&redis, &id, "tree").await
}

pub async fn add_module(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, AWError> {
    let (hashes_id, blob_name, blob_id) = path.into_inner();

    let mut hashes: Hashes = match redis_helper::get_slice(&hashes_id, "tree", &redis).await {
        Some(t) => serde_json::from_slice(&t).expect("Hashes are Serializable"),
        None => {
            log::debug!("hashes not found");
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    // check if blob_id exists;
    let domain = format!("blob:{}", &blob_id);
    let blob_exists = redis.send(Command(resp_array!["EXISTS", &domain])).await?;

    match blob_exists {
        Ok(RespValue::Integer(x)) if x == 1 => (),
        Ok(RespValue::Integer(x)) if x == 0 => {
            log::debug!("module:{} did not exist", &blob_id);
            return Ok(HttpResponse::NotFound().finish());
        }
        _ => {
            log::error!("error looking up module:{}", &blob_id);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    }

    // we avoid going to db if there is no change
    let prev_id = &hashes.id();
    hashes.insert(blob_name, blob_id);

    if prev_id == &hashes.id() {
        let result = json!({"status":"ok", "id":&prev_id, "mes":"no change in hash id"});
        return Ok(HttpResponse::Ok().json(result));
    }

    match redis_helper::add(&hashes, &redis).await {
        true => {
            // tree will be updated
            let result = json!({"status":"ok", "id":&hashes.id()});
            Ok(HttpResponse::Ok().json(result))
        }
        false => {
            log::error!("failed to add updated tree");
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn add_modules(
    redis: web::Data<Addr<RedisActor>>,
    tree_id: web::Path<String>,
    other: web::Json<Tree>,
) -> Result<HttpResponse, AWError> {
    let tree_id = tree_id.into_inner();

    let mut tree: Tree = match redis_helper::get_slice(&tree_id, "tree", &redis).await {
        Some(t) => serde_json::from_slice(&t).expect("Tree is Serializable"),
        None => {
            log::debug!("tree:{} not found", &tree_id);
            return Ok(HttpResponse::NotFound().finish());
        }
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
        log::debug!("some blob id did not exist");
        return Ok(HttpResponse::NotFound().finish());
    }

    let prev_id = &tree.id();
    tree.assign(&other);
    
    // we avoid going to db if there is no change
    if prev_id == &tree.id() {
        let result = json!({"status":"ok", "id":&prev_id, "mes":"no change in hash id"});
        return Ok(HttpResponse::Ok().json(result));
    }

    match redis_helper::add(&tree, &redis).await {
        true => {
            // tree will be updated
            let result = json!({"status":"ok", "id":&tree.id()});
            Ok(HttpResponse::Ok().json(result))
        }
        false => {
            log::error!("failed to add updated tree");
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn remove_module(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AWError> {
    let (tree_id, blob_name) = path.into_inner();

    let mut tree: Tree = match redis_helper::get_slice(&tree_id, "tree", &redis).await {
        Some(t) => serde_json::from_slice(&t).expect("Tree is Serializable"),
        None => {
            log::debug!("tree:{} not found", &tree_id);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    match tree.remove(&blob_name) {
        None => {
            let result = json!({"status":"ok", "id":&tree.id(), "mes":"no change in hash id"});
            return Ok(HttpResponse::Ok().json(result));
        }
        Some(_x) => {
            match redis_helper::add(&tree, &redis).await {
                true => {
                    // tree will be updated
                    let result = json!({"status":"ok", "id":&tree.id()});
                    Ok(HttpResponse::Ok().json(result))
                }
                false => {
                    log::error!("failed to add updated tree");
                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
        }
    }
}

pub async fn post(
    redis: web::Data<Addr<RedisActor>>,
    payload: web::Json<Tree>,
) -> Result<HttpResponse, AWError> {
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
        return Ok(HttpResponse::NoContent().body("some blob does not exist"));
    }

    redis_helper::post(&redis, &b).await
}
