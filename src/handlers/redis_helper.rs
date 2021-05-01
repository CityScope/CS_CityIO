use jct::Settable;
use actix::Addr;
use actix_redis::{Command, RedisActor, RespValue as Value};
use actix_web::web;
use futures::future::{join, join_all};
use redis_async::resp_array;

// TODO this is obscuring the error, not best practice
pub async fn add(obj: &impl Settable, redis: &web::Data<Addr<RedisActor>>) -> bool {
    let add = redis.send(Command(resp_array!["SET", obj.domain(), obj.json()]));

    let plural_domain = format!("{}s", obj.prefix());

    let list = redis.send(Command(resp_array![
        "SADD",
        &plural_domain,
        &obj.id()
    ]));

    let (add, _list) = join(add, list).await;

    if let Ok(Ok(Value::SimpleString(x))) = add {
        return x == "OK";
    }

    false
}

pub async fn get_slice(
    id: &str,
    domain_prefix: &str,
    redis: &web::Data<Addr<RedisActor>>,
) -> Option<Vec<u8>> {
    let domain = format!("{}:{}", domain_prefix, id);

    let obj = redis.send(Command(resp_array!["GET", &domain])).await;

    if let Ok(Ok(Value::BulkString(x))) = obj {
        Some(x)
    } else {
        None
    }
}

pub async fn get_list(
    plural: &str,
    redis: &web::Data<Addr<RedisActor>>,
) -> Option<Vec<String>> {

    let set = redis.send(Command(resp_array!["SMEMBERS", plural])).await;

    match set {
        Ok(Ok(Value::Array(x))) => {
            let mut result = Vec::new();

            for e in x {
                if let Value::BulkString(x) = e {
                    let id = String::from_utf8(x)
                        .expect("id should be utf-8");
                    result.push(id);
                }
            }

            Some(result)
        }
        _ => return None,
    }
}

pub async fn get_slices(
    ids: &Vec<String>,
    domain_prefix: &str,
    redis: &web::Data<Addr<RedisActor>>,
) -> Vec<Vec<u8>> {
    let get_list = join_all(ids.iter().map(|id| {
        let domain = format!("{}:{}", domain_prefix, id);
        redis.send(Command(resp_array!["GET", &domain]))
    }))
    .await;

    let mut slices = Vec::new();

    for obj in get_list {
        if let Ok(Ok(Value::BulkString(x))) = obj {
            slices.push(x)
        }
    }

    slices
}
