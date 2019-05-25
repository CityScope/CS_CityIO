use crate::model::{JSONObject, JSONState, Meta};
use actix_web::http::{header, StatusCode};
use actix_web::{get, web, Error, HttpResponse, Result as ActixResult};
use futures::{Future, Stream};
use log::{debug, warn};
use serde_json::{json, Value};
use url::Url;

const CITY_SCOPE: &str = "http://cityscope.media.mit.edu/CS_CityIO_Frontend/";
const BASE_URL: &str = "https://cityio.media.mit.edu";

#[get("/")]
pub fn index() -> ActixResult<HttpResponse> {
    let redirect_url = Url::parse(CITY_SCOPE).unwrap();

    Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
        .header(header::LOCATION, redirect_url.as_str())
        .finish())
}

pub fn list_tables(
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |_| {
        let tables = state.lock().unwrap();

        let mut names: Vec<String> = Vec::new();
        let mut url = Url::parse(BASE_URL).unwrap();

        for key in tables.keys() {
            url.set_path(&format!("api/table/{}", &key.to_string()));
            names.push(url.as_str().to_string());
        }

        Ok(HttpResponse::Ok().json(&names))
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

        debug!("/n **get_table** /n{:?}/n", &name);

        let data = match map.get(&name) {
            Some(v) => v,
            None => {
                let mes = format!("table '{}' not found", &name);
                return Ok(HttpResponse::Ok().json(err_json(&mes)));
            }
        };
        Ok(HttpResponse::Ok().json(&data))
    })
}

pub fn get_table_field(
    path: web::Path<(String, String)>,
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |_| {
        let name = format!("{}", path.0);
        let map = state.lock().unwrap();

        debug!("/n **get_table_field** /n{:?}/n", &name);

        let table_data = match map.get(&name) {
            Some(v) => v,
            None => {
                let mes = format!("table '{}' not found", &name);
                return Ok(HttpResponse::Ok().json(err_json(&mes)));
            }
        };

        let mut field = format!("{}", path.1);

        // is field empty??
        if &field == "" {
            return Ok(HttpResponse::Ok().json(&table_data));
        }

        if field.chars().last().unwrap() == '/' {
            field.pop();
        }

        let fields = field.split("/");
        let mut data: Value = table_data.to_owned();

        for f in fields {
            match data.get(&f) {
                Some(d) => data = d.to_owned(),
                None => {
                    let mes = format!("table '{}' does not include field '{}'", &name, &f);
                    return Ok(HttpResponse::Ok().json(err_json(&mes)));
                }
            }
        }
        Ok(HttpResponse::Ok().json(&data))
    })
}

pub fn set_table(
    name: web::Path<String>,
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        // body is loaded, now we can deserialize json-rust

        debug!("/n **set_table** /n{:?}/n", &body);

        let mut result: JSONObject = match serde_json::from_slice(&body) {
            Ok(v) => v,
            Err(e) => {
                let mes = format!("error parsing to json: {}", e.to_string());
                warn!("json parse error.");
                return Ok(HttpResponse::Ok().json(err_json(&mes)));
            }
        };

        let name = format!("{}", *name);

        let mut map = state.lock().unwrap();
        let meta = Meta::new(&json!(result).to_string());
        result.insert("meta".to_string(), json!(meta));
        map.insert(name, json!(result));

        Ok(HttpResponse::Ok().json(json!({"status":"ok"})))
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
        Ok(HttpResponse::Ok().json(json!({"status":"ok"})))
    })
}

////////////////////////
// helpers
////////////////////////
fn err_json(mes: &str) -> Value {
    json!({
        "status": "error",
        "mes" : mes
    })
}
