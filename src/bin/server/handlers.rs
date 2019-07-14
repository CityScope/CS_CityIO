use crate::model::{JSONObject, JSONState, Meta};
use actix_web::http::{header, StatusCode};
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Result as ActixResult};
use base64::decode;
use futures::future::ok as fut_ok;
use futures::{Future, Stream};
use log::{debug, warn};
use serde_json::{from_str, json, Map, Value};
use std::str;
use std::sync::mpsc;
use std::thread;
use url::Url;

const CITY_SCOPE: &str = "https://cityscope.media.mit.edu/CS_CityIO/";
const BASE_URL: &str = "https://cityio.media.mit.edu";

const BLACK_LIST_TABLE: [&str; 2] = ["clear", "update"];
const BLACK_LIST_MODULE: [&str; 1] = ["meta"];

#[get("/")]
pub fn index() -> ActixResult<HttpResponse> {
    let redirect_url = Url::parse(CITY_SCOPE).unwrap();

    Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
        .header(header::LOCATION, redirect_url.as_str())
        .finish())
}

pub fn list_tables(state: web::Data<JSONState>) -> impl Future<Item = HttpResponse, Error = Error> {
    let tables = state.lock().unwrap();

    let mut names: Vec<String> = Vec::new();
    let mut url = Url::parse(BASE_URL).unwrap();

    for key in tables.keys() {
        url.set_path(&format!("api/table/{}", &key.to_string()));
        names.push(url.as_str().to_string());
    }

    fut_ok(HttpResponse::Ok().json(&names))
}

pub fn get_table(
    name: web::Path<String>,
    state: web::Data<JSONState>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let name = format!("{}", *name);

    for b in &BLACK_LIST_TABLE {
        if b == &name {
            let mes = format!("table name: {} is not allowed", &name);
            return fut_ok(not_acceptable(&mes));
        }
    }

    let map = state.lock().unwrap();

    debug!(" **get_table** {:?}", &name);

    let data = match map.get(&name) {
        Some(v) => v,
        None => {
            let mes = format!("table '{}' not found", &name);
            return fut_ok(not_acceptable(&mes));
        }
    };

    let user = match data.get("header").and_then(|header| header.get("user")) {
        Some(user) => format!("{}", user).replace("\"", ""),
        None => return fut_ok(HttpResponse::Ok().json(&data)),
    };

    debug!("user: {:?}", user);

    let header = &req.headers();

    match header.get("token") {
        Some(token) => {
            debug!("token {:?}", &token);
            let token_str = token.to_str().unwrap();
            if token_str == &user {
                fut_ok(HttpResponse::Ok().json(&data))
            } else {
                fut_ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish())
            }
        }
        None => fut_ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish()),
    }

    // fut_ok(HttpResponse::Ok().json(&data))
}

pub fn deep_get(
    path: web::Path<(String, String)>,
    state: web::Data<JSONState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let (name, mut field) = path.to_owned();

    for b in &BLACK_LIST_TABLE {
        if b == &name {
            let mes = format!("table name: {} is not allowed", &name);
            return fut_ok(not_acceptable(&mes));
        }
    }

    let map = state.lock().unwrap();

    debug!("**get_table_field** {:?}", &name);

    let table_data = match map.get(&name) {
        Some(v) => v,
        None => {
            let mes = format!("table '{}' not found", &name);
            return fut_ok(not_acceptable(&mes));
        }
    };

    // is field empty??
    if &field == "" {
        return fut_ok(HttpResponse::Ok().json(&table_data));
    }

    if field.chars().last().unwrap() == '/' {
        field.pop();
    }

    let fields = field.split("/");
    let mut data: &Value = table_data;

    for f in fields {
        match data.get(&f) {
            Some(d) => data = d,
            None => {
                let mes = format!("data in table '{}' does not include field '{}'", &name, &f);
                return fut_ok(not_acceptable(&mes));
            }
        }
    }
    fut_ok(HttpResponse::Ok().json(&data))
}

pub fn set_table(
    name: web::Path<String>,
    state: web::Data<JSONState>,
    pl: web::Payload,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        // body is loaded, now we can deserialize json-rust

        let name = format!("{}", *name);

        for b in &BLACK_LIST_TABLE {
            if b == &name {
                let mes = format!("table name: {} is not allowed", &name);
                return Ok(not_acceptable(&mes));
            }
        }

        debug!("**set_table** {:?}", &name);

        let mut result: JSONObject = match serde_json::from_slice(&body) {
            Ok(v) => v,
            Err(e) => {
                let mes = format!("error parsing to json: {}", &e);
                warn!("json parse error.");
                return Ok(not_acceptable(&mes));
            }
        };

        match &result.get("header").and_then(|h| h.get("user")) {
            Some(user) => {
                let user = user.to_string();
                let headers = &req.headers();
                let token = match headers.get("token") {
                    None => {
                        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
                    },
                    Some(t) => format!("{:?}",t)
                };

                debug!("{:?}, {:?}", &user, &token);

                //TODO: lookup token and verify
                if token != user {
                    return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
                }
            }
            None => (),
        }

        thread::spawn(move || {
            let mut map = state.lock().unwrap();

            let mut new_table = match map.get(&name) {
                Some(t) => {
                    let mut tmp: Map<String, Value> = t.as_object().unwrap().to_owned();
                    result.remove("meta");
                    for (k, v) in result.iter() {
                        tmp.insert(k.to_owned(), v.to_owned());
                    }
                    tmp
                }
                None => result,
            };

            // let meta = Meta::new(&format!("{:?}", &new_table));
            let meta = Meta::from_map(&new_table);
            new_table.insert("meta".to_owned(), json!(&meta));

            // let meta = Meta::new(&json!(result).to_string());
            // result.insert("meta".to_string(), json!(meta));
            map.insert(name, json!(&new_table));
        });

        Ok(HttpResponse::Ok().json(json!({"status":"ok"})))
    })
}

pub fn set_module(
    path: web::Path<(String, String)>,
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        // body is loaded, now we can deserialize json-rust

        let (table_name, module_name) = path.to_owned();

        for b in &BLACK_LIST_TABLE {
            if b == &table_name {
                let mes = format!("table name: {} is not allowed", &table_name);
                return Ok(not_acceptable(&mes));
            }
        }

        for b in &BLACK_LIST_MODULE {
            if b == &module_name {
                let mes = format!("module name: {} is not allowed", &module_name);
                return Ok(not_acceptable(&mes));
            }
        }

        debug!("**set_module** {}/{}", &table_name, &module_name);

        let result: Value = match serde_json::from_slice(&body) {
            Ok(v) => v,
            Err(e) => {
                let mes = format!("error parsing to json: {}", &e);
                return Ok(not_acceptable(&mes));
            }
        };

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let meta = Meta::new(str::from_utf8(&body).unwrap());
            tx.send(meta.id()).unwrap(); // -->
        });

        let mut map = state.lock().unwrap();

        let mut current = match map.get(&table_name) {
            Some(t) => t.as_object().unwrap().to_owned(),
            None => Map::new(),
        };

        // current.remove(&"meta".to_string());
        current.insert(module_name.to_owned(), result);

        // let mut meta = json!(Meta::new(&format!("{:?}", &current)));
        let mut meta: Meta = match current.get("meta") {
            Some(m) => from_str(&m.to_string()).unwrap(),
            None => Meta::new(""),
        };

        let module_hash = rx.recv().unwrap(); // <--
        meta.hashes.insert(module_name.to_owned(), module_hash);
        meta.update();

        debug!("{:?}", &meta);

        current.insert("meta".to_string(), json!(meta));
        map.insert(table_name, json!(current));

        Ok(HttpResponse::Ok().json(json!({"status":"ok"})))
    })
}

pub fn clear_table(
    name: web::Path<String>,
    state: web::Data<JSONState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let name = format!("{}", *name);
    let mut map = state.lock().unwrap();
    map.remove(&name);
    fut_ok(HttpResponse::Ok().json(json!({"status":"ok"})))
}

////////////////////////
// auth
////////////////////////

struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(combined: &str) -> User {
        let sp: Vec<&str> = combined.split(":").collect();

        User {
            username: sp[0].to_owned(),
            password: sp[1].to_owned(),
        }
    }
}

pub fn auth(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    let headers = req.headers();

    let user = match headers.get("Authorization") {
        Some(h) => {
            let auth_header = format!("{:?}", &h).replace("\"", "");
            let split: Vec<&str> = auth_header.split(" ").collect();
            let user_str = String::from_utf8(decode(&split[1]).unwrap()).unwrap();
            User::new(&user_str)
        }
        None => {
            return fut_ok(
                HttpResponse::Ok()
                    .json(json!({"status": "'authenticate' field not found in header"})),
            )
        }
    };
    fut_ok(HttpResponse::Ok().json(json!({"token": &user.username})))
}

////////////////////////
// helpers
////////////////////////
fn not_acceptable(mes: &str) -> HttpResponse {
    HttpResponse::NotAcceptable().json(&json!({
        "status": "error",
        "mes" : &mes}
    ))
}
