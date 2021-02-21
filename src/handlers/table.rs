use crate::{
    model::{Table, TempTable, Settable, Module},
    redis_helper::{redis_add, redis_delete, redis_get_slice},
};
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{web, Error as AWError, HttpResponse};
use futures::future::{join, join_all};
use redis_async::{resp::RespValue, resp_array};
use std::collections::BTreeMap;
use std::iter::FromIterator;
use serde_json::Value;

const BLACK_LIST_MODULE: [&str; 1] = ["meta"];

pub async fn get(
    redis: web::Data<Addr<RedisActor>>,
    table_name: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let get_hash = redis_get_slice(&table_name.into_inner(), "head", &redis).await;

    let id = match get_hash {
        Some(h) => {
            String::from_utf8(h).expect("this is base58 data")
        },
        _=>{
            return Ok(HttpResponse::NoContent().body("table name not in head"));
        }
    };

    let get_table = redis_get_slice(&id, "table", &redis).await;
    
    let table:Table = match get_table {
        Some(v) => serde_json::from_slice(&v).expect("This should be Deserializable"),
        None => {return Ok(HttpResponse::NoContent().body("table id not found in db"))}
    };

    let compiled = table.compile(&redis).await;
    
    Ok(HttpResponse::Ok().json(compiled))
}

pub async fn list_head(
    redis: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {

    let list = redis.send(Command(resp_array![
        "SMEMBERS",
        "heads"
    ])).await?;

    if let Ok(RespValue::Array(ids)) = list {
        let mut list: Vec<String> = Vec::new();
        
        // TODO: this can be filter + map
        for id in ids {
            if let RespValue::BulkString(v) = id {
                let head = String::from_utf8(v).expect("table name should be utf8");
                list.push(head);
            }
        }

        Ok(HttpResponse::Ok().json(list))
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

pub async fn deep_get(
    redis: web::Data<Addr<RedisActor>>,
    path: web::Path<(String, String)>
) -> Result<HttpResponse, AWError> {

    let (table_name, tail) = path.into_inner();

    // get table id
    let get_id = redis_get_slice(&table_name, "head", &redis).await;

    let table_id = match get_id {
        Some(v) => {
            String::from_utf8(v).expect("table_id is base58")
        },
        _=> {
            return Ok(HttpResponse::NoContent().body("table name not found in heads"));
        }
    };

    let get_table = redis_get_slice(&table_id, "table", &redis).await;

    let table: Table = match get_table{
        Some(v) => serde_json::from_slice(&v).expect("table should be Deserializable"),
        None =>{ 
            return Ok(HttpResponse::NoContent().body("table id not found in db"));
        }
    };
    
    // we should compile the table

    // check if there is a module

    log::debug!("table id: {}", &table_id);

    // get table


    let dirs: Vec<String> = tail
    .split("/")
    .filter(|s|s!=&"")
    .map(|s|s.to_string())
    .collect();

    log::debug!("path:{:?}", dirs);

    Ok(HttpResponse::Ok().body("ok"))
}

pub async fn post(
    redis: web::Data<Addr<RedisActor>>,
    table_name: web::Path<String>,
    table: web::Json<Value>,
) -> Result<HttpResponse, AWError> {

    // partial data needs to be an JSON object 
    let table_data = table.into_inner(); 
    let table_name = table_name.into_inner();

    let mut prev_table: Option<Table> = None;
    // check if we have the previous table
    let get_head_id = redis_get_slice(&table_name, "head", &redis)
        .await
        .and_then(|s| Some(String::from_utf8(s).expect("should be base58")));
    
    if let Some(v) = get_head_id {
        prev_table = redis_get_slice(&v, "table", &redis).await.and_then(|s|{
            let table:Table = serde_json::from_slice(&s).expect("Should be Serializable");
            Some(table)
        })  
    }

    // we can assume that the modules are all in the database at this point for the previous table
    
    let data: BTreeMap<String, Value> = match table_data{
       Value::Object(v) => {
           BTreeMap::from_iter(v.iter()
               .map(|(k, v)|(k.to_string(), v.to_owned())))
       },
       _ => {
           return Ok(HttpResponse::NotAcceptable().body("json payload needs to be a JSON object"));
       }
    };

    let modules: Vec<Module> = data.iter()
        .filter(|(k,_v)|{
           !BLACK_LIST_MODULE.iter().any(|b| b==k)
        })
        .map(|(k,v)|{
        Module::new(k, v)
    }).collect();


    let module_hashes: BTreeMap<String,String> = modules.iter().map(|m|(m.name(), m.id())).collect();

    // get only new hashes, if this is empty we are happy to return as-is.
    // note that the previous table may have more modules.
    // this post function is more like git add that doesn't take account removes
    let mut new_modules: Vec<String>;
    let mut new_module_names: Vec<String>;

    match prev_table.as_ref() {
        None => {
            new_modules = module_hashes.values().map(|id|id.to_string()).collect();
            new_module_names = module_hashes.keys().map(|name| name.to_string()).collect();
        },
        Some(v) => {
            new_modules = module_hashes
                .values()
                .filter(|nh| !v.hashes.values().any(|ph| &ph==nh))
                .map(|id|id.to_string())
                .collect();
            new_module_names = module_hashes
                .keys()
                .filter(|nh| !v.hashes.keys().any(|ph| &ph==nh))
                .map(|id|id.to_string())
                .collect();
        }
    }

    if (new_modules.is_empty()) && (new_module_names.is_empty()) {
        // we know that there was an previous table
        let prev_id = prev_table.unwrap().hash.to_string();
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "id": prev_id,
            "name": &table_name,
            "status": "ok",
            "mes": "status up to date, nothing to add"
        })));
    }

    let mut union_hash = match prev_table.as_ref() {
        Some(t) => t.hashes.to_owned(),
        None => BTreeMap::new()
    };
        
    for (k,v) in &module_hashes {
        union_hash.insert(k.to_string(), v.to_string());
    }

    let mod_add = modules.iter()
        .filter(|m|{new_modules.iter().any(|id| id==&m.id())}) // just add what is needed
        .map(|m|{redis_add(m.to_owned(), &redis)});

    let prev_id = prev_table.as_ref().and_then(|pt| Some(pt.hash.to_string()));

    let table: Table = TempTable::new(prev_id, union_hash, table_name.to_string()).into();
    let hash: String = table.hash.to_string();
    let name: String = table.table_name.to_string();

    let head = redis.send(Command(resp_array![
            "SET",
            format!("head:{}",&table.table_name),
            &table.id()
    ]));

    let heads_list = redis.send(Command(resp_array!["SADD", "heads", &table_name]));
    let add = redis_add(table, &redis);
    let (add, _head) = join(add, join(join_all(mod_add), join(head, heads_list))).await;
    
    match add {
        true => Ok(HttpResponse::Ok().json(
                serde_json::json!(
                    {
                        "id":hash,
                        "name":name,
                        "status":"ok"
                    }))),
        false => Ok(HttpResponse::InternalServerError().body("could not add table to database"))
    }
}

// pub async fn deep_post(
//     redis: web::Data<Addr<RedisActor>>,
//     table_name: web::Path<String>,
//     table: web::Json<Module>,
// ) -> Result<HttpResponse, AWError> {
//     Ok(HttpResponse::Ok().body("ok"))
// }


