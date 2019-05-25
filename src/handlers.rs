use crate::database;
use crate::model::{CityIOState, Meta, NewTable};
use std::sync::mpsc;
use std::thread;

use actix_web::http::{header, StatusCode};
use actix_web::{get, web, Error, HttpResponse, Result as ActixResult};
use chrono::Utc;
use futures::future::{ok as fut_ok, Either};
use futures::{Future, Stream};
use log::{debug, warn};
use serde_json::{from_slice, from_value, json, Map, Value};
use url::Url;

type JSONObject = Map<String, Value>;

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

pub fn get_table(
    path: web::Path<String>,
    state: web::Data<CityIOState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match lookup_table_and_add(&path.to_owned(), state) {
        Some(t) => fut_ok(HttpResponse::Ok().json(t)),
        None => fut_ok(HttpResponse::Ok().json("table not found")),
    }
}

// deep get
pub fn get_table_field(
    path: web::Path<(String, String)>,
    state: web::Data<CityIOState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let table_name = path.0.to_owned();
    let table_data = match lookup_table_and_add(&table_name, state) {
        Some(t) => t,
        None => return fut_ok(HttpResponse::Ok().json("table not found")),
    };

    let mut field = path.1.to_owned();
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

pub fn update_module(
    path: web::Path<(String, String)>,
    state: web::Data<CityIOState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let field_name = path.1.to_owned();

    if &field_name == "meta" {
        let mes = format!(
            "Posting a module with name '{}' is not allowed. Request was rejected.",
            &field_name
        );
        return Either::A(fut_ok(HttpResponse::Ok().json(err_json(&mes))));
    }

    Either::B(pl.concat2().from_err().and_then(move |body| {
        // get the table data
        let (tx, rx) = mpsc::channel();

        let thread_state = state.clone();
        let table_name = path.0.to_owned();
        thread::spawn(move || {
            let tmp = lookup_table_and_add(&table_name, thread_state);
            tx.send(tmp).unwrap();
        });

        let module_data = match from_slice(&body) {
            Ok(d) => d,
            Err(e) => return Ok(HttpResponse::Ok().json(e.to_string())),
        };

        let rec = rx.recv().unwrap();
        let mut table_data: JSONObject;

        match rec {
            Some(t) => table_data = from_value(t).unwrap(),
            None => table_data = Map::new(),
        };

        table_data.insert(field_name, module_data);
        let new_id = table_data.update_meta();

        let thread_state = state.clone();
        let table_name = path.0.to_owned();
        let data_for_db = json!(&table_data);
        // fire and forget or should we ??
        thread::spawn(move || {
            let state_data = thread_state.lock().unwrap();
            let pool = state_data.pool.to_owned();
            let now = Utc::now().naive_utc();
            let new = NewTable {
                id: &new_id,
                ts: &now,
                name: &table_name,
                data: &data_for_db,
            };
            if database::create_table(new, pool).is_err() {
                warn!("could not save to database, not saving");
            };
        });

        let mut state = state.lock().unwrap();

        let tables = &mut state.tables;
        let table_name = path.0.to_owned();

        tables.insert(table_name, json!(&table_data));

        Ok(HttpResponse::Ok().json(&table_data))
    }))
}

pub fn set_table(
    name: web::Path<String>,
    state: web::Data<CityIOState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        // body is loaded, now we can deserialize json-rust

        // table name 'clear' is not allowed
        let name = name.to_owned();

        if &name == "clear" || &name == "update" {
            let mes = format!(
                "Posting a table with name '{}' is not allowed. Request was rejected.",
                &name
            );
            return Ok(HttpResponse::Ok().json(err_json(&mes)));
        }

        debug!("**set_table** {:?}", &body);

        let mut result: JSONObject = match from_slice(&body) {
            Ok(v) => v,
            Err(e) => {
                let mes = format!("error parsing to json: {}", e.to_string());
                warn!("json parse error.");
                return Ok(HttpResponse::Ok().json(err_json(&mes)));
            }
        };

        let mut state_data = state.lock().unwrap();

        let new_id = result.update_meta();
        state_data.tables.insert(name.clone(), json!(result));

        let data_for_db: Value = json!(result.to_owned());

        let state = state.clone();

        // fire and forget or should we ??
        thread::spawn(move || {
            let state_data = state.lock().unwrap();
            let pool = state_data.pool.to_owned();
            let now = Utc::now().naive_utc();
            let new = NewTable {
                id: &new_id,
                ts: &now,
                name: &name,
                data: &data_for_db,
            };
            if database::create_table(new, pool).is_err() {
                warn!("could not save to db");
            };
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

trait MetaUpdatable {
    fn update_meta(&mut self) -> String;
}

impl MetaUpdatable for JSONObject {
    fn update_meta(&mut self) -> String {
        self.remove("meta");
        let s = json!(&self).to_string();
        let meta = Meta::new(&s);
        self.insert("meta".to_string(), json!(&meta));
        meta.id
    }
}

/// looks for the table in both memory and db.
/// if it finds in the db, and not in memory,
/// it adds it to memory.
fn lookup_table_and_add(name: &str, state: web::Data<CityIOState>) -> Option<Value> {
    let (tx, rx) = mpsc::channel();

    let table_name = name.to_owned();
    let thread_state = state.clone();

    thread::Builder::new()
        .name("lookup_table_and_add".to_string())
        .spawn(move || {
            let state = thread_state.lock().expect("cannot lock state??");
            let tmp = database::read_last_table(&table_name, &state.pool).map(|t| t.data);
            if tx.send(tmp).is_err() {
                warn!("wasted channel");
            }
        })
        .unwrap();

    let mut state = state.lock().unwrap();
    let tables = &mut state.tables;
    let table_name = name.to_owned();

    match &tables.get(&table_name) {
        // we found it in memory
        Some(t) => Some(json!(t)),
        None => {
            // let's check the db.
            let rec = rx.recv().unwrap();
            match rec {
                Ok(t) => {
                    tables.insert(table_name, t.to_owned());
                    Some(t)
                }
                Err(e) => {
                    debug!("{}", &e.to_string());
                    None
                }
            }
        }
    }
}
