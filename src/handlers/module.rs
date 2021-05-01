use crate::model::Module;
use crate::{redis_helper, json_error};

use actix::prelude::*;
use actix_redis::RedisActor;
use actix_web::{Responder, get, http::StatusCode, post, web};
use serde_json::{json, Value};

#[get("api/module/{id}/")]
pub async fn get(redis: web::Data<Addr<RedisActor>>, id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    match redis_helper::get_slice(&id, "blob", &redis).await {
        Some(v)=> {
            let v:Value = serde_json::from_slice(&v).unwrap();
            web::Json(v).with_status(StatusCode::OK)
        },
        None => {
            json_error("cannot find module").with_status(StatusCode::NOT_FOUND)
        }
    }
}

#[post("api/module/")]
pub async fn post(
    redis: web::Data<Addr<RedisActor>>,
    payload: web::Json<Module>,
) -> impl Responder {
    let m: Module = payload.into_inner();
    match redis_helper::add(&m, &redis).await {
        true => {
            let result = json!({"status":"ok"});
            web::Json(result).with_status(StatusCode::OK)
        },
        false => {
            json_error("could not post module").with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
