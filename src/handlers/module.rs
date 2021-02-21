use crate::{
    // model::{Table, PartialTable, Settable, Module},
    model::{Module, Settable},
    redis_helper::{redis_add, redis_delete, redis_get_slice},
};
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{web, Error as AWError, HttpResponse};
use futures::future::join;
use redis_async::{resp::RespValue, resp_array};
use std::collections::BTreeMap;

pub async fn get(
    redis: web::Data<Addr<RedisActor>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AWError> {

    let id = id.into_inner();

    let slice = redis_get_slice(&id, "module", &redis).await;

    match slice {
        Some(v) => {
            let module: Module = serde_json::from_slice(&v)
                .expect("this should be Serializable");
            Ok(HttpResponse::Ok().json(module))
        },
        None => {
            let mes = format!("id {} not found", &id);
            Ok(HttpResponse::NotFound().body(&mes))
        }
    }
}

pub async fn post(
    redis: web::Data<Addr<RedisActor>>,
    name: web::Path<String>,
    payload: web::Json<serde_json::Value>,
) -> Result<HttpResponse, AWError> {
    
    let payload = payload.into_inner();
    let name = name.into_inner();
    let module = Module::new(&name, &payload);
    let id = module.id();
    let send = redis_add(module, &redis).await;

    match send {
        true => {
            log::debug!("ok");
            Ok(HttpResponse::Ok().json(serde_json::json!({"status":"ok", "id":id})))
        },
        false => Ok(HttpResponse::InternalServerError().body("could not add module to db"))
    }
}

