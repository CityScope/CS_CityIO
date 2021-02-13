use crate::{
    model::{Table, PartialTable, Settable},
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
    table_name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().body("ok"))
}

pub async fn list_head(
    redis: web::Data<Addr<RedisActor>>,
    table_name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().body("ok"))
}

pub async fn deep_get(
    redis: web::Data<Addr<RedisActor>>,
    table_name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().body("ok"))
}

pub async fn post(
    redis: web::Data<Addr<RedisActor>>,
    table_name: web::Path<String>,
    table: web::Json<PartialTable>,
) -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().body("ok"))
}

pub async fn deep_post(
    redis: web::Data<Addr<RedisActor>>,
    table_name: web::Path<String>,
    table: web::Json<PartialTable>,
) -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().body("ok"))
}


