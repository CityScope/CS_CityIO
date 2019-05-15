use log::{warn, debug};
use actix_web::{get, web, Error, HttpResponse, Result as ActixResult};
use actix_web::http::{header, StatusCode};
use futures::{Future, Stream};
use serde_json::{json, Error as JSONError};

use crate::model::{TableList, Meta, JSONState, JSONObject};

const REDIRECT_URL: &str = "http://cityscope.media.mit.edu/CS_CityIO_Frontend/";
const BASE_URL: &str = "https://cityio.media.mit.edu/";

#[get("/")]
pub fn index() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
       .header(header::LOCATION, REDIRECT_URL)
       .finish())
}

pub fn list_tables(
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |_| {
        let tables = state.lock().unwrap();

        let mut names: Vec<String> = Vec::new();

        for key in tables.keys() {
            names.push(format!("{}api/table/{}", &BASE_URL, &key.to_string()));
        }

        let data = json!(TableList::new(names)).to_string();
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(data.into_bytes()))
    })
}

pub fn get_table(
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

pub fn set_table(
    name: web::Path<String>,
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        // body is loaded, now we can deserialize json-rust
        
        debug!("{:?}", &body);

        let result: Result<JSONObject, JSONError> =
            serde_json::from_slice(&body); // return Result

        let name = format!("{}", *name);

        let mut injson: JSONObject = match result {
            Ok(v) => v,
            Err(_e) => {
                let mut er = JSONObject::new();
                er.insert("status".to_string(), json!("error, please post a Json **Object**"));
                warn!("error in json format");
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

pub fn clear_table(
    name: web::Path<String>,
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |_| {
        
        let name = format!("{}", *name);
        let mut map = state.lock().unwrap();
        map.remove(&name);
        let status_success = json!({
            "status": "ok",
        });
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(status_success.to_string().into_bytes()))
    })
}
