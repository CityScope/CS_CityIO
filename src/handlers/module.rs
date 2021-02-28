use crate::redis_helper;
use crate::model::Module;

use actix::prelude::*;
use actix_redis::RedisActor;
use actix_web::{web, Error as AWError, HttpResponse};

pub async fn get(
    redis: web::Data<Addr<RedisActor>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let id = id.into_inner();
    redis_helper::get(&redis, &id, "blob").await
}

pub async fn post(
    redis: web::Data<Addr<RedisActor>>,
    payload: web::Json<Module>,
) -> Result<HttpResponse, AWError> {
    let m: Module = payload.into_inner();
    redis_helper::post(&redis, &m).await
}

