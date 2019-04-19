extern crate json;
#[macro_use]
extern crate actix_web;

use serde_derive::{Deserialize, Serialize};

use actix_web::{
    web, App, Error, HttpResponse, HttpServer, Result as ActixResult};
use actix_web::http::{header, StatusCode};
use actix_web::middleware::{cors::Cors, Logger};
use futures::{Future, Stream};
use serde_json::{json, Map, Value, Error as JSONError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use sha256::sha256::{format_hash, hash};

type JSONState = Arc<Mutex<HashMap<String, Value>>>;
type JSONObject = Map<String, Value>;

#[get("/")]
fn index() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
       .header(header::LOCATION,"http://cityscope.media.mit.edu/CS_CityIO_Frontend/")
       .finish())
}

/// gets table data by asking the name of the table
fn get_table(
    name: web::Path<String>,
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |_| {
        let name = format!("{}", *name);
        let map = state.lock().unwrap();
        let mut data: String;

        match map.get(&name) {
            Some(v) => data = v.to_string(),
            None => data = json!({"status": "table not found"}).to_string(),
        };

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(data.into_bytes()))
    })
}

#[derive(Serialize, Deserialize)]
struct TableList(Vec<String>);

#[derive(Serialize, Deserialize)]
struct Meta {
    id: String,
    timestamp: u64,
    apiv: String,
}

impl Meta {
    fn new(dump: &str) -> Meta {
        let id = format_hash(&hash(&dump));
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let apiv = String::from("2.0");

        Meta {
            id: id,
            timestamp: now,
            apiv: apiv,
        }
    }
}

/// gets table data by asking the name of the table
fn list_tables(
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |_| {
        let tables = state.lock().unwrap();

        let mut names: Vec<String> = Vec::new();

        for key in tables.keys() {
            names.push(format!("https://cityio.media.mit.edu/api/table/{}", &key.to_string()));
        }

        let data = json!(TableList(names)).to_string();
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(data.into_bytes()))
    })
}

/// sets the table to /table/{name}
fn set_table(
    name: web::Path<String>,
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        // body is loaded, now we can deserialize json-rust
        let result: Result<JSONObject, JSONError> =
            serde_json::from_slice(&body); // return Result

        let name = format!("{}", *name);

        let mut injson: JSONObject = match result {
            Ok(v) => v,
            Err(_e) => {
                let mut er = JSONObject::new();
                er.insert("status".to_string(), json!("error, please post a Json **Object**"));
                er
            }
        };

        let mut map = state.lock().unwrap();
        let meta = Meta::new(&json!(injson).to_string());
        injson.insert(String::from("meta"), json!(meta));
        map.insert(name, json!(injson));

        let status_success = json!({
            "status": "ok",
        });

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(status_success.to_string().into_bytes()))
    })
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let hashmap: JSONState = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            // enable logger
            .data(hashmap.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST"])
                    .send_wildcard()
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ]),
            )
            .service(
                web::resource("/api/table/{name}")
                    .route(web::get().to_async(get_table))
            )
            .service(
                web::resource("/api/table/update/{name}")
                    .route(web::post().to_async(set_table))
            )
            .service(web::resource("/api/tables/list").route(web::get().to_async(list_tables)))
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
}
