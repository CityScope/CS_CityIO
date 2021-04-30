use crate::model::{get_redis, set_redis, Hashes, Meta, Module, Table};
use crate::redis_helper;
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{web, Error as AWError, HttpResponse};
use futures::future::{join, join_all};
use jct::{Blob, Commit, Settable, Tag};
use redis_async::{resp::RespValue, resp_array};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::mem::swap;

pub async fn get_raw(
    redis: web::Data<Addr<RedisActor>>,
    name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let name = name.into_inner();
    redis_helper::get(&redis, &name, "tag").await
}

pub async fn list(redis: web::Data<Addr<RedisActor>>) -> HttpResponse {
    match redis_helper::get_list("tags", &redis).await {
        None => HttpResponse::InternalServerError().finish(),
        Some(list) => HttpResponse::Ok().json(list),
    }
}

pub async fn delete(redis: web::Data<Addr<RedisActor>>, name: web::Path<String>) -> HttpResponse {
    let name = name.into_inner();

    let tag_domain = format!("tag:{}", name);

    let delete_tag = redis.send(Command(resp_array!["DEL", &tag_domain]));
    let remove_from_tags = redis.send(Command(resp_array!["SREM", "tags", name]));

    let (del, _rm) = join(delete_tag, remove_from_tags).await;

    match del {
        Ok(Ok(RespValue::Integer(x))) => {
            if x == 1 {
                HttpResponse::Ok().body("ok")
            } else {
                HttpResponse::Ok().body("no change")
            }
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_module(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (name, module_name) = path.into_inner();

    let (_, mut commit, mut hashes) = match table_commit_hashes(&name, &redis).await {
        None => return HttpResponse::NotFound().finish(),
        Some(i) => i,
    };

    if let None = hashes.remove(&module_name) {
        return HttpResponse::Ok().body("module not found, no change");
    }

    commit.update_tree(&hashes.id());

    match update(hashes, commit, &name, &redis).await {
        None => HttpResponse::InternalServerError().finish(),
        Some(c_id) => {
            let result = json!({"status":"ok", "commit_id":c_id});
            HttpResponse::Ok().json(result)
        }
    }
}

pub async fn post_raw(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AWError> {
    let (name, commit_id) = path.into_inner();

    let commit: Commit = match redis_helper::get_slice(&commit_id, "commit", &redis).await {
        Some(s) => serde_json::from_slice(&s).expect("Commit has Deserialize"),
        None => {
            log::debug!("could not find commit:{}", &commit_id);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let t = Tag::new(&name, &commit.id());
    redis_helper::post(&redis, &t).await
}

pub async fn get(
    redis: web::Data<Addr<RedisActor>>,
    name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let name = name.into_inner();

    match unroll_table(&name, &redis).await {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(t) => Ok(HttpResponse::Ok().json(t)),
    }
}

pub async fn deep_get(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AWError> {
    let (table_name, tail) = path.into_inner();

    let table_data = match unroll_table(&table_name, &redis).await {
        Some(t) => t,
        None => {
            log::debug!("could not unroll table.");
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let dirs: Vec<String> = tail
        .split("/")
        .filter(|s| s != &"")
        .map(|s| s.to_string())
        .collect();

    log::debug!("path:{:?}", dirs);

    let mut value =
        serde_json::to_value(table_data).expect("BTreeMap should be convertable to Json Value");

    for d in dirs {
        value = match value.get(d) {
            Some(v) => v.to_owned(),
            None => return Ok(HttpResponse::NotFound().finish()),
        }
    }

    Ok(HttpResponse::Ok().json(value))
}

pub async fn post(
    redis: web::Data<Addr<RedisActor>>,
    name: web::Path<String>,
    data: web::Json<BTreeMap<String, Value>>,
) -> HttpResponse {
    let name = name.into_inner();

    let mut data: BTreeMap<String, Blob> = data
        .into_inner()
        .iter()
        .filter(|(name, _v)| name != &"meta")
        .map(|(name, value)| (name.to_string(), value.to_owned()))
        .collect();

    let new_hashes: Hashes = data
        .iter()
        .map(|(name, blob)| (name.to_string(), blob.id()))
        .collect();

    let mut merged_hashes: Hashes = Hashes::new();

    let previous = table_commit_hashes(&name, &redis).await;

    let commit: Commit;

    if previous.is_some() {
        let (_, p_commit, p_hashes) = previous.unwrap();

        merged_hashes = p_hashes.to_owned();

        for (name, hash) in new_hashes {
            merged_hashes.insert(name, hash);
        }

        if merged_hashes.id() == p_hashes.id() {
            return HttpResponse::Ok().body("no change");
        }

        commit = Commit::new(&merged_hashes.id(), &Some(&p_commit.id()));

        data = data
            .iter()
            .filter(|(_name, blob)| p_hashes.values().any(|ph| ph != &blob.id()))
            .map(|(name, blob)| (name.to_owned(), blob.to_owned()))
            .collect();
    } else {
        merged_hashes = new_hashes;
        commit = Commit::new(&merged_hashes.id(), &None);
    }

    // add things
    let table: Table = Table::new(&name, &commit.id());
    let add_table = set_redis(table, &redis);
    let new_commit_id = commit.id();
    let add_commit = set_redis(commit, &redis);
    let add_hashes = set_redis(merged_hashes, &redis);
    let add_blobs = data
        .values()
        .map(|value| set_redis(value.to_owned(), &redis));

    let ((table, commit), (hashes, blobs)) = join(
        join(add_table, add_commit),
        join(add_hashes, join_all(add_blobs)),
    )
    .await;

    if !table || !commit || !hashes || !blobs.iter().any(|b| *b) {
        HttpResponse::InternalServerError().finish()
    } else {
        let result = json!({"status":"ok", "id":new_commit_id, "name": name});
        HttpResponse::Ok().json(result)
    }
}

pub async fn deep_post(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>,
    data: web::Json<Value>,
) -> HttpResponse {
    let (table_name, tail) = path.into_inner();
    let mut data = data.into_inner();

    let (_, mut commit, mut hashes) = match table_commit_hashes(&table_name, &redis).await {
        None => return HttpResponse::NotFound().finish(),
        Some((t, c, h)) => (t, c, h),
    };

    let mut dirs: Vec<String> = tail
        .split("/")
        .filter(|s| s != &"")
        .map(|s| s.to_string())
        .collect();

    let mut dirs_iter = dirs.iter();

    let module_name = match dirs_iter.next() {
        Some(t) => t.to_owned(),
        None => return HttpResponse::NotAcceptable().finish(),
    };

    if module_name == "meta" {
        log::debug!("meta is reserved module name, not allowed");
        return HttpResponse::NotAcceptable().finish();
    }

    dirs = dirs_iter.map(|d| d.to_string()).collect();

    let module_id = match hashes.get(&module_name) {
        Some(v) => v,
        None => {
            let module = data;

            hashes.insert(module_name, module.id());
            commit.update_tree(&hashes.id());

            let add_module = set_redis(module, &redis);
            let update = update(hashes, commit, &table_name, &redis);

            let (_mod, update) = join(add_module, update).await;

            if let Some(v) = update {
                let result = json!({"status":"ok", "id": v});
                return HttpResponse::Ok().json(result);
            } else {
                return HttpResponse::Ok().body("will add module");
            }
        }
    };

    let mut module: Module = match get_redis(&module_id, "blob", &redis).await {
        Some(v) => v,
        None => return HttpResponse::NotFound().finish(),
    };

    let mut partial = &mut module;

    for dir in dirs {
        partial = match partial.get_mut(dir) {
            Some(v) => v,
            None => return HttpResponse::NotFound().finish(),
        }
    }

    swap(partial, &mut data);

    if &module.id() == module_id {
        let json = serde_json::json!({
            "status":"ok",
            "mes": "no change on table"
        });
        return HttpResponse::Ok().json(json);
    }

    hashes.insert(module_name, module.id());
    commit.update_tree(&hashes.id());

    let add_module = set_redis(module, &redis);
    let commit_id = update(hashes, commit, &table_name, &redis);

    let (_add_module, add_others) = join(add_module, commit_id).await;

    if let Some(id) = add_others {
        let result = json!({"status":"ok", "commit":id});
        HttpResponse::Ok().json(result)
    } else {
        log::debug!("failed updating");
        HttpResponse::InternalServerError().finish()
    }
}

pub async fn deep_delete(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AWError> {

    let (table_name, tail) = path.into_inner();

    let (_, mut commit, mut hashes) = match table_commit_hashes(&table_name, &redis).await {
        None => return Ok(HttpResponse::NotFound().finish()),
        Some((t, c, h)) => (t, c, h),
    };

    let mut dirs: Vec<String> = tail
        .split("/")
        .filter(|s| s != &"")
        .map(|s| s.to_string())
        .collect();

    let mut dirs_iter = dirs.iter();

    let module_name = match dirs_iter.next() {
        Some(t) => t.to_owned(),
        None => return Ok(HttpResponse::NotAcceptable().finish()),
    };

    if module_name == "meta" {
        log::debug!("meta is reserved module name, not allowed");
        return Ok(HttpResponse::NotAcceptable().finish());
    }

    dirs = dirs_iter.map(|d| d.to_string()).collect();

    if dirs.len() == 0 {
        // wants to delete the module
        match hashes.remove(&module_name) {
            Some(_v) => {
                () 
            },
            None => {
                return Ok(HttpResponse::Ok().json(json!({"status":"ok", "mes":"module not found, state did not change."})))  
            }
        }
    } else {
        // deeper
    let module_id = match hashes.get(&module_name) {
        Some(v) => v,
        None => {
            return Ok(HttpResponse::Ok().json(json!({"status":"ok", "mes":"did not find module name, table state did not change."})))  
        }
    };

    let mut module: Module = match get_redis(&module_id, "blob", &redis).await {
        Some(v) => v,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let mut partial = &mut module;

    for dir in dirs {
        partial = match partial.get_mut(dir) {
            Some(v) => v,
            None => return Ok(HttpResponse::NotFound().finish()),
        }
    }

    swap(partial, &mut Value::Null);

    if &module.id() == module_id {
        let json = serde_json::json!({
            "status":"ok",
            "mes": "no change on table"
        });
        return Ok(HttpResponse::Ok().json(json));
    }

    hashes.insert(module_name, module.id());
    let _add_module = set_redis(module, &redis).await;
    }

    commit.update_tree(&hashes.id());
    let commit_id = update(hashes, commit, &table_name, &redis).await;

    if let Some(id) = commit_id {
        let result = json!({"status":"ok", "commit":id});
        Ok(HttpResponse::Ok().json(result))
    } else {
        log::debug!("failed updating");
        Ok(HttpResponse::InternalServerError().finish())
    }

}


async fn update(
    hashes: Hashes,
    commit: Commit,
    table_name: &str,
    redis: &web::Data<Addr<RedisActor>>,
) -> Option<String> {
    let c_id = commit.id();
    let add_hashes = set_redis(hashes, redis);
    let add_commit = set_redis(commit, redis);
    let table: Table = Table::new(table_name, &c_id);
    let add_table = set_redis(table, redis);

    let (did_hash, (did_commit, did_table)) = join(add_hashes, join(add_commit, add_table)).await;

    if did_hash && did_commit && did_table {
        return Some(c_id.to_string());
    } else {
        return None;
    }
}

fn to_hashes(data: BTreeMap<String, Value>) -> Hashes {
    data.iter()
        .map(|(v, b)| {
            let blob: Blob = b.to_owned();
            (v.to_string(), blob.id())
        })
        .collect()
}

async fn table_commit_hashes(
    table_name: &str,
    redis: &web::Data<Addr<RedisActor>>,
) -> Option<(Table, Commit, Hashes)> {
    let table: Table = match get_redis(&table_name, "tag", &redis).await {
        None => return None,
        Some(t) => t,
    };

    let commit: Commit = match get_redis(&table.commit, "commit", &redis).await {
        None => return None,
        Some(t) => t,
    };

    match get_redis(&commit.tree(), "tree", &redis).await {
        None => None,
        Some(t) => Some((table, commit, t)),
    }
}

async fn unroll_table(
    table_name: &str,
    redis: &web::Data<Addr<RedisActor>>,
) -> Option<BTreeMap<String, Value>> {
    let table: Table = match get_redis(&table_name, "tag", &redis).await {
        Some(t) => t,
        None => {
            log::debug!("table name ({}) was not found", &table_name);
            return None;
        }
    };

    let commit: Commit = match get_redis(&table.commit, "commit", &redis).await {
        Some(t) => t,
        None => {
            log::debug!("commit ({}) was not found", &table.commit);
            return None;
        }
    };

    let hashes: Hashes = match get_redis(&commit.tree(), "tree", &redis).await {
        Some(t) => t,
        None => {
            log::debug!("tree ({}) was not found", &commit.tree());
            return None;
        }
    };

    let blobs: Vec<Option<Blob>> =
        join_all(hashes.values().map(|id| get_redis(id, "blob", &redis))).await;

    if blobs.iter().any(|b| b.is_none()) {
        log::debug!("blob not found");
        return None;
    }

    let blobs: Vec<Blob> = blobs
        .iter()
        .map(|b| b.as_ref().unwrap().to_owned())
        .collect();

    let meta = Meta::new(&table.commit, &commit.tree(), &hashes, &commit.timestamp());

    let mut result: BTreeMap<String, Value> = BTreeMap::new();

    for (i, (name, _id)) in hashes.iter().enumerate() {
        result.insert(name.to_string(), blobs[i].to_owned());
    }

    result.insert(
        "meta".to_string(),
        serde_json::to_value(meta).expect("meta can be Value"),
    );

    Some(result)
}
