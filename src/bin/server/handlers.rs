use crate::model::{JSONObject, JSONState, JsonUser, Meta};
use crate::Pool;
use actix_web::http::{header, header::HeaderMap, StatusCode};
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Result as ActixResult};
use base64::decode;
use cs_cityio_backend::auth_user;
use futures::future::ok as fut_ok;
use futures::{Future, Stream};
use log::{debug, warn};
use serde::Deserialize;
use serde_json::{from_str, json, Map, Value};
use std::collections::HashMap;
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
    let map = state.lock().unwrap();
    let tables = map.get("tables").unwrap();
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
    let name = name.to_owned();

    for b in &BLACK_LIST_TABLE {
        if b == &name {
            let mes = format!("table name: {} is not allowed", &name);
            return fut_ok(not_acceptable(&mes));
        }
    }

    let map = state.lock().unwrap();
    let tables = map.get("tables").unwrap();

    debug!(" **get_table** {:?}", &name);

    let data = match tables.get(&name) {
        Some(v) => v,
        None => {
            let mes = format!("table '{}' not found", &name);
            return fut_ok(not_acceptable(&mes));
        }
    };

    let table_user = match data.get("header").and_then(|header| header.get("user")) {
        Some(user) => format!("{}", user).replace("\"", ""),
        // public
        None => return fut_ok(HttpResponse::Ok().json(&data)),
    };

    let users = map.get("users").unwrap();
    let header = &req.headers();

    if check_auth(&table_user, &header, users) {
        fut_ok(HttpResponse::Ok().json(&data))
    } else {
        fut_ok(un_authed("access restricted"))
    }
}

pub fn check_auth(table_user: &str, header: &HeaderMap, users: &HashMap<String, Value>) -> bool {
    let token = match header.get("Authorization") {
        Some(t) => t.to_str().unwrap(),
        None => return false,
    };

    let split: Vec<&str> = token.split_whitespace().collect();

    println!("{:?}", split);

    // if split.len() <= 2 || split[0] != "Bearer" {
    //     return false;
    // }

    // let tkn = &split[1].to_owned();

    let tkn = match &split.get(1) {
        Some(&t) => t,
        None => return false,
    };

    debug!("{}", &tkn);

    let user: JsonUser = match users.get(tkn) {
        Some(t) => serde_json::from_str(&t.to_string()).unwrap(),
        None => return false,
    };

    user.name == table_user || user.is_super
}

pub fn deep_get(
    path: web::Path<(String, String)>,
    state: web::Data<JSONState>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let (name, mut field) = path.to_owned();

    for b in &BLACK_LIST_TABLE {
        if b == &name {
            let mes = format!("table name: {} is not allowed", &name);
            return fut_ok(not_acceptable(&mes));
        }
    }

    let map = state.lock().unwrap();
    let tables = map.get("tables").unwrap();

    debug!("**get_table_field** {:?}", &name);

    let table_data = match tables.get(&name) {
        Some(v) => v,
        None => {
            let mes = format!("table '{}' not found", &name);
            return fut_ok(not_acceptable(&mes));
        }
    };

    match &table_data.get("header").and_then(|h| h.get("user")) {
        None => (), // public
        Some(u) => {
            let users = map.get("users").unwrap();
            let header = &req.headers();
            if !check_auth(u.as_str().unwrap(), &header, users) {
                return fut_ok(un_authed("access restricted"));
            }
        }
    }

    // is field empty??
    if &field == "" {
        return fut_ok(HttpResponse::Ok().json(&table_data));
    }

    if field.ends_with('/') {
        field.pop();
    }

    let fields = field.split('/');
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

        let name = name.to_owned();

        let tmp_state = state.to_owned();
        let map = tmp_state.lock().unwrap();

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
            None => (),
            Some(u) => {
                let table_user = u.as_str().unwrap();
                let users = map.get("users").unwrap();
                let headers = &req.headers();
                if !check_auth(table_user, headers, users) {
                    return Ok(un_authed("access restricted"));
                }
            }
        }

        thread::spawn(move || {
            let mut map = state.lock().unwrap();
            let tables = map.get_mut("tables").unwrap();

            let mut new_table = match tables.get(&name) {
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
            tables.insert(name, json!(&new_table));
        });

        Ok(HttpResponse::Ok().json(json!({"status":"ok"})))
    })
}

pub fn set_module(
    path: web::Path<(String, String)>,
    state: web::Data<JSONState>,
    pl: web::Payload,
    req: HttpRequest,
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
        let users = map.get("users").unwrap().to_owned();
        let tables = &mut map.get_mut("tables").unwrap();

        let mut current = match tables.get(&table_name) {
            Some(t) => t.as_object().unwrap().to_owned(),
            None => Map::new(),
        };

        match &current.get("header").and_then(|h| h.get("user")) {
            None => (),
            Some(u) => {
                let table_user = u.as_str().unwrap();
                let headers = &req.headers();
                if !check_auth(table_user, headers, &users) {
                    return Ok(un_authed("access restricted"));
                }
            }
        }

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
        tables.insert(table_name, json!(current));

        Ok(HttpResponse::Ok().json(json!({"status":"ok"})))
    })
}

pub fn clear_module(
    path: web::Path<(String, String)>,
    state: web::Data<JSONState>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error>{
    let (table_name, module_name) = path.to_owned();

    let mut map = state.lock().unwrap();
    let user_map = map.to_owned();
    let users = user_map.get("users").unwrap();
    let tables = map.get_mut("tables").unwrap();

    let table_data = tables.get_mut(&table_name)
                           .and_then(|x| x.as_object_mut())
                           .unwrap();

    match &table_data.get("header").and_then(|h| h.get("user")) {
        None => (), // public
        Some(u) => {
            let header = &req.headers();
            if !check_auth(u.as_str().unwrap(), &header, users) {
                return fut_ok(un_authed("access restricted"));
            }
        }
    }

    if table_data.contains_key(&module_name) {
        table_data.remove(&module_name);
        let hashes = table_data.get_mut("meta")
                               .and_then(|x| x.get_mut("hashes"))
                               .and_then(|x| x.as_object_mut())
                               .unwrap();
        hashes.remove(&module_name);
    }

    fut_ok(HttpResponse::Ok().json(json!({"status":"ok"})))
}

pub fn clear_table(
    name: web::Path<String>,
    state: web::Data<JSONState>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let name = name.to_owned();
    let mut map = state.lock().unwrap();
    let user_map = map.to_owned();
    let users = user_map.get("users").unwrap();
    let tables = map.get_mut("tables").unwrap();

    let table_data = &tables.get(&name).unwrap();

    match &table_data.get("header").and_then(|h| h.get("user")) {
        None => (), // public
        Some(u) => {
            let header = &req.headers();
            if !check_auth(u.as_str().unwrap(), &header, users) {
                return fut_ok(un_authed("access restricted"));
            }
        }
    }

    tables.remove(&name);
    fut_ok(HttpResponse::Ok().json(json!({"status":"ok"})))
}

////////////////////////
// auth
////////////////////////

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

impl User {
    pub fn decode_base64(&self) -> Result<(String, String), &'static str> {
        let d_name: String = match decode(&self.username) {
            Ok(d) => String::from_utf8(d).unwrap(),
            Err(_) => return Err("could not decode name"),
        };

        let d_pass: String = match decode(&self.password) {
            Ok(d) => String::from_utf8(d).unwrap(),
            Err(_) => return Err("could not decode password"),
        };

        Ok((d_name, d_pass))
    }
}

pub fn auth(
    pool: web::Data<Pool>,
    pl: web::Payload,
    req: web::HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        let con = &pool.get().unwrap();

        // FIXME: we no longer need this
        // let credential: User = match serde_json::from_slice(&body){
        //     Ok(c) => c,
        //     Err(_) => return fut_ok(un_authed("could not parse payload to json")),
        // };

        let headers = req.headers();

        let username = match headers.get("username") {
            Some(u) => u.to_str().unwrap(),
            None => return fut_ok(un_authed("username not found in header"))
        };

        let password = match headers.get("password") {
            Some(p) => p.to_str().unwrap(),
            None => return fut_ok(un_authed("password not found in header"))
        };
        
        // // TODO: each value wil be base64 encoded
        // let (u, p) = match credential.decode_base64() {
        //     Ok(r) => r,
        //     Err(e) => return fut_ok(un_authed(e)),
        // };

        match auth_user(con, username, password) {
        // match auth_user(con, &credential.username, &credential.password) {
            Some(u) => {
                // TODO give a list of authed tables
                let usr = json!({"user": u.username, "id": u.id, "token": u.hash, "is_super": u.is_super});
                fut_ok(HttpResponse::Ok().json(usr))
            },
            None => fut_ok(un_authed("user not found")),
        }
    })
}

////////////////////////
// helpers
////////////////////////

fn un_authed(mes: &str) -> HttpResponse {
    HttpResponse::build(StatusCode::UNAUTHORIZED).json(json!(&mes))
}

fn not_acceptable(mes: &str) -> HttpResponse {
    HttpResponse::NotAcceptable().json(&json!({
        "status": "error",
        "mes" : &mes}
    ))
}
