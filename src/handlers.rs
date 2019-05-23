use crate::database;
use crate::model::{JSONObject, JSONState, Meta, NewTable};
use crate::CityIOState;
use std::collections::HashMap;
use std::thread;

use actix_web::http::{header, StatusCode};
use actix_web::{get, web, Error, HttpResponse, Result as ActixResult};
use chrono::prelude::*;
use chrono::Utc;
use futures::future::{ok as fut_ok, Either};
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

// this only lists the table in memory,
// rather than the whole collection in the db
pub fn list_tables(
    state: web::Data<CityIOState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |_| {
        let state = &state.lock().unwrap();
        let tables = &state.tables;

        let mut names: Vec<String> = Vec::new();
        let mut url = Url::parse(BASE_URL).unwrap();

        for key in tables.keys() {
            url.set_path(&format!("api/table/{}", &key.to_string()));
            names.push(url.as_str().to_string());
        }

        Ok(HttpResponse::Ok().json(&names))
    })
}

// gets table from memory and db. First it will look into the HashMap
// if not found, digs into the db, if found adds to the memory
pub fn get_table(
    name: web::Path<String>,
    io: web::Data<CityIOState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let table_name = format!("{}", name);
    let pool: database::Pool;
    {
        let state = io.lock().unwrap();
        pool = state.pool.to_owned();
        tables = state.tables.to_owned();
    }

    match tables.get(&table_name) {
        // we found it in memory!
        Some(t) => Either::A(fut_ok(HttpResponse::Ok().json(t))),
        // we dig into the db
        None => Either::B(
            web::block(move || database::read_last_table_named(&table_name, pool, io)).then(
                |res| match res {
                    // found it
                    Ok(t) => Ok(HttpResponse::Ok().json(t.data)),
                    // nope, or an error occured
                    Err(e) => Ok(HttpResponse::Ok().json(err_json(&e.to_string()))),
                },
            ),
        ),
    }
}

// deep get
pub fn get_table_field(
    path: web::Path<(String, String)>,
    state: web::Data<CityIOState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let name = format!("{}", path.0);
    let pool: database::Pool;
    let tables: HashMap<String, Value>;
    {
        let state = state.lock().unwrap();
        pool = state.pool.to_owned();
        tables = state.tables.to_owned();
    }

    let mut field = format!("{}", path.1);

    debug!("**get_table_field** {:?}", &name);

    let table_data: Value;

    match tables.get(&name) {
        Some(v) => {
            table_data = v.to_owned();
        }
        None => {
            let table_in_db =
                web::block(move || database::read_last_table_named(&name, pool, state)).wait();
            match table_in_db {
                Ok(t) => {
                    table_data = t.data;
                }
                Err(e) => {
                    let mes = format!("{:?}", e.to_string());
                    return fut_ok(HttpResponse::Ok().json(err_json(&mes)));
                }
            }
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
    let mut data: Value = table_data.to_owned();

    for f in fields {
        match data.get(&f) {
            Some(d) => data = d.to_owned(),
            None => {
                let mes = format!("table does not include field '{}'", &f);
                return fut_ok(HttpResponse::Ok().json(err_json(&mes)));
            }
        }
    }

    fut_ok(HttpResponse::Ok().json(&data))
}

pub fn set_table(
    name: web::Path<String>,
    state: web::Data<CityIOState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        // body is loaded, now we can deserialize json-rust

        // table name 'clear' is not allowed
        let name = format!("{}", *name);

        if &name == "clear" {
            return Ok(HttpResponse::Ok()
                .json("Posting a table with name 'clear' is not allowed. Request was rejected."));
        }

        debug!("**set_table** {:?}", &body);

        let mut result: JSONObject = match serde_json::from_slice(&body) {
            Ok(v) => v,
            Err(e) => {
                let mes = format!("error parsing to json: {}", e.to_string());
                warn!("json parse error.");
                return Ok(HttpResponse::Ok().json(err_json(&mes)));
            }
        };

        let mut state_data = state.lock().unwrap();

        // formulating the meta field
        let meta = Meta::new(&json!(result).to_string());
        result.insert("meta".to_string(), json!(meta));
        state_data.tables.insert(name.clone(), json!(result));

        let data_for_db: Value = json!(result.to_owned());

        let state = state.clone();

        // fire and forget or should we ??
        thread::spawn(move || {
            let state_data = state.lock().unwrap();
            let pool = state_data.pool.to_owned();
            let now = Utc::now().naive_utc();
            let new = NewTable {
                id: &meta.id,
                ts: &now,
                name: &name,
                data: &data_for_db,
            };
            database::create_table(new, pool);
        });

        Ok(HttpResponse::Ok().json(json!({"status":"ok"})))
    })
}

pub fn clear_table(
    name: web::Path<String>,
    state: web::Data<CityIOState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |_| {
        let name = format!("{}", *name);

        {
            let mut state = state.lock().unwrap();
            state.tables.remove(&name);
        }

        let state = state.clone();

        thread::spawn(move || {
            let state = state.lock().unwrap();
            let pool = state.pool.to_owned();
            let _ = database::drop_table_named(&name, pool);
        });

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
