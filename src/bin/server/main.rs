mod handlers;
mod model;

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use actix_web::http::{header};
use actix_web::middleware::{Logger, NormalizePath};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use log::info;

use handlers::{auth, clear_table, get_table, deep_get, index, list_tables, set_module, set_table};
use model::JSONState;

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

    info!("starting server @ {}", &port);

    let hashmap: JSONState = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .data(hashmap.clone())
            .wrap(Logger::default())
            .wrap(NormalizePath)
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
                web::resource("/api/table/update/{name}/").route(web::post().to_async(set_table)),
            )
            .service(
                web::resource("/api/table/update/{name}/{module}").route(web::post().to_async(set_module)),
            )
            .service(
                web::resource("/api/table/update/{name}/{module}/").route(web::post().to_async(set_module)),
            )
            .service(
                web::resource("/api/table/clear/{name}").route(web::get().to_async(clear_table)),
            )
            .service(
                web::resource("/api/table/clear/{name}/").route(web::get().to_async(clear_table)),
            )
            .service(web::resource("/api/tables/list/").route(web::get().to_async(list_tables)))
            .service(web::resource("/api/tables/list").route(web::get().to_async(list_tables)))
            .service(
                web::resource("/api/table/{name}/{tail:.*}")
                    .route(web::get().to_async(deep_get)),
            )

            .service(
                web::resource("/users/authenticate")
                    .route(web::get().to_async(list_tables))
                    .route(web::post().to_async(auth))
            )
            .service(index)
                // fs::Files::new("/", "./static").index_file("index.html"),
    })
    .bind(format!("127.0.0.1:{}", &port))?
    .run()
}
