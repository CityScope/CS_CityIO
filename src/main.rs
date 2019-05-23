#[macro_use]
extern crate diesel;

mod database;
mod handlers;
mod model;
mod schema;

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use actix_web::http::header;
use actix_web::middleware::{cors::Cors, Logger};
use actix_web::{web, App, HttpServer};
use log::info;

use database::{init_pool, Pool};
use handlers::{clear_table, get_table, get_table_field, index, list_tables, set_table};
use model::JSONState;
use serde_json::{Value};

pub struct CityIOData {
    pool: Pool,
    tables: HashMap<String, Value>,
}

impl CityIOData {
    pub fn new () -> Self {
        CityIOData {
            pool: init_pool(),
            tables: HashMap::new()
        }
    }
}

pub type CityIOState = Arc<Mutex<CityIOData>>;

fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "actix_web=info,cs_cityio_backend=debug");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info,cs_cityio_backend=info");
    }

    env_logger::init();

    let port: String;

    match env::args().nth(1) {
        Some(new_port) => port = new_port,
        None => port = "8080".to_string(),
    }

    // data, accessible to each handler
    // let hashmap: JSONState = Arc::new(Mutex::new(HashMap::new()));
    // let pool:Pool = init_pool();

    let state: CityIOState = Arc::new(Mutex::new(CityIOData::new()));

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
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
            .service(web::resource("/api/table/{name}").route(web::get().to_async(get_table)))
            .service(
                web::resource("/api/table/update/{name}").route(web::post().to_async(set_table)),
            )
            .service(
                web::resource("/api/table/clear/{name}").route(web::get().to_async(clear_table)),
            )
            .service(web::resource("/api/tables/list").route(web::get().to_async(list_tables)))
            .service(
                web::resource("/api/table/{name}/{tail:.*}")
                    .route(web::get().to_async(get_table_field)),
            )
            .service(index)
    })
    .bind(format!("127.0.0.1:{}", &port))
    .and_then(|result| {
        info!("server started, running @ {}", &port);
        Ok(result)
    })?
    .run()
}
