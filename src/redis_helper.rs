use crate::model::Settable;
use actix::Addr;
use actix_redis::{Command, RedisActor};
use actix_web::web;
use futures::future::{join, join_all};
use redis_async::{resp::RespValue as Value, resp_array};

// TODO this is obscuring the error, not best practice
pub async fn redis_add(obj: impl Settable, redis: &web::Data<Addr<RedisActor>>) -> bool {
    let add = redis.send(Command(resp_array!["SET", obj.domain(), obj.json()]));

    let plural_domain = format!("{}s", obj.prefix());

    let list = redis.send(Command(resp_array![
        "SADD",
        &plural_domain,
        &obj.list_item()
    ]));

    let (add, _list) = join(add, list).await;

    if let Ok(Ok(Value::SimpleString(x))) = add {
        return x == "OK";
    }

    false
}

// to delete the object, you need to get it first
// in order to remove it from the SET
pub async fn redis_delete(obj: impl Settable, redis: &web::Data<Addr<RedisActor>>) -> bool {
    let del = redis.send(Command(resp_array!["DEL", obj.domain()]));
    let plural_domain = format!("{}s", obj.domain());
    let pop = redis.send(Command(resp_array![
        "SREM",
        &plural_domain,
        &obj.list_item()
    ]));

    let (del, _) = join(del, pop).await;

    if let Ok(Ok(Value::Integer(x))) = del {
        return x == 1;
    }

    false
}

pub async fn redis_get_slice(
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

pub async fn redis_get_list(
    domain: &str,
    redis: &web::Data<Addr<RedisActor>>,
) -> Option<Vec<(String, String)>> {
    let plural = format!("{}s", domain);

    let set = redis.send(Command(resp_array!["SMEMBERS", &plural])).await;

    match set {
        Ok(Ok(Value::Array(x))) => {
            let mut result = Vec::new();

            for e in x {
                if let Value::BulkString(x) = e {
                    let list_item: (String, String) = serde_json::from_slice(&x)
                        .expect("data format for list item should be (String, String)");
                    result.push(list_item);
                }
            }

            Some(result)
        }
        _ => return None,
    }
}

pub async fn redis_get_slices(
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
