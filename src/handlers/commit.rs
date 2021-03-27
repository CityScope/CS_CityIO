use jct::{TempCommit, Commit, Settable};
use crate::redis_helper;
use actix::prelude::*;
use actix_redis::RedisActor;
use actix_web::{web, Error as AWError, HttpResponse};
use serde_json::json;

pub async fn get(
    redis: web::Data<Addr<RedisActor>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let id = id.into_inner();
    redis_helper::get(&redis, &id, "commit").await
}

pub async fn post(
    redis: web::Data<Addr<RedisActor>>,
    payload: web::Json<TempCommit>,
) -> Result<HttpResponse, AWError> {
    let c : Commit = payload.into_inner().into();
    redis_helper::post(&redis, &c).await
}

pub async fn update_tree(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>
) -> Result<HttpResponse, AWError> {
    let (commit_id, tree_id) = path.into_inner();

    let mut commit:Commit = match redis_helper::get_slice(&commit_id, "commit", &redis).await {
        Some(s) => 
            serde_json::from_slice(&s).expect("Commit is Deserializeable"),
        None => {
            log::debug!("commit id:{} wasn't found", &commit_id);
            return Ok(HttpResponse::NotFound().finish())
        }
    };
    
    match commit.update_tree(&tree_id) {
       Some(v) => {
            match  redis_helper::add(&commit, &redis).await {
                true => {
                    let result = json!({
                        "status":"ok",
                        "id": &commit.id(),
                        "prev_id": &v
                    });
                    Ok(HttpResponse::Ok().json(result))
                },
                false => {
                    log::error!("error adding commit");
                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
       },
       None => {
            let result = json!({"status":"ok", "id":&commit_id, "mes":"no change in tree id"});
            Ok(HttpResponse::Ok().json(result))
       }
    }
}

