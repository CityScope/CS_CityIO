use crate::{json_error, redis_helper};
use actix::prelude::*;
use actix_redis::RedisActor;
use actix_web::{get, http::StatusCode, post, web, Responder};
use jct::{Commit, Settable, TempCommit};
use serde_json::{from_slice, json};

#[get("api/commit/{id}/")]
pub async fn get(redis: web::Data<Addr<RedisActor>>, id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    match redis_helper::get_slice(&id, "commit", &redis).await {
        Some(v) => {
            let v: serde_json::Value = from_slice(&v).unwrap();
            web::Json(v).with_status(StatusCode::OK)
        }
        None => json_error("could not find commit").with_status(StatusCode::NOT_FOUND),
    }
}

#[post("api/commit/")]
pub async fn post(
    redis: web::Data<Addr<RedisActor>>,
    payload: web::Json<TempCommit>,
) -> impl Responder {
    let c: Commit = payload.into_inner().into();
    match redis_helper::add(&c, &redis).await {
        true => {
            let result = json!({"status":"ok"});
            web::Json(result).with_status(StatusCode::OK)
        }
        false => json_error("could not add commit").with_status(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[post("api/commit/{id}/{tree_id}/")]
pub async fn update_tree(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (commit_id, tree_id) = path.into_inner();

    let mut commit: Commit = match redis_helper::get_slice(&commit_id, "commit", &redis).await {
        Some(s) => serde_json::from_slice(&s).expect("Commit is Deserializeable"),
        None => {
            log::debug!("commit id:{} wasn't found", &commit_id);
            return json_error("commit not found").with_status(StatusCode::NOT_FOUND);
        }
    };

    match commit.update_tree(&tree_id) {
        Some(v) => match redis_helper::add(&commit, &redis).await {
            true => {
                let result = json!({
                    "status":"ok",
                    "id": &commit.id(),
                    "prev_id": &v
                });
                web::Json(result).with_status(StatusCode::OK)
            }
            false => {
                json_error("error adding commit").with_status(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        None => {
            let result = json!({"status":"ok", "id":&commit_id, "mes":"no change in tree id"});
            web::Json(result).with_status(StatusCode::OK)
        }
    }
}
