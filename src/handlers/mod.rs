pub mod user;
pub mod table;

use actix::Addr;
use actix_redis::{Command, RedisActor};
use actix_web::{web, Error as AWError, Responder};
use redis_async::{resp::RespValue, resp_array};

pub async fn nuclear(redis: web::Data<Addr<RedisActor>>) -> Result<impl Responder, AWError> {
    let res = redis.send(Command(resp_array!["FLUSHALL",])).await?;

    match res {
        Ok(RespValue::SimpleString(x)) if x == "OK" => Ok("ok"),
        _ => Ok("nope"),
    }
}

