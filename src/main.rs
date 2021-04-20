mod handlers;
mod model;

use actix_cors::Cors;
use actix_redis::RedisActor;
use actix_web::{middleware, web, App, HttpServer};
use dotenv;
use std::env;

use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace,cityio=debug");
    env_logger::init();

    HttpServer::new(|| {
        let address = format!(
            "{}:{}",
            env::var("REDIS_ADDR").unwrap(),
            env::var("REDIS_PORT").unwrap()
        );

        let redis_addr = RedisActor::start(&address);

        // TODO: change this
        let cors = Cors::permissive();

        App::new()
            .data(redis_addr)
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 4))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(web::resource("/api/dump/").route(web::get().to(dump)))
            .service(web::resource("/api/restore/").route(web::post().to(restore)))
            .service(web::resource("/api/nuclear/").route(web::delete().to(nuclear)))
            .service(web::resource("/api/module/{id}/").route(web::get().to(module::get)))
            .service(web::resource("/api/module/").route(web::post().to(module::post)))
            .service(
                web::resource("/api/hashes/{id}/{blob_name}/")
                    .route(web::delete().to(hashes::remove_module)),
            )
            .service(web::resource("/api/hashes/{id}/").route(web::get().to(hashes::get)))
            .service(web::resource("/api/hashes/").route(web::post().to(hashes::post)))
            // commit
            .service(
                web::resource("/api/commit/{id}/{tree_id}/")
                    .route(web::post().to(commit::update_tree)),
            )
            .service(web::resource("/api/commit/{id}/").route(web::get().to(commit::get)))
            .service(web::resource("/api/commit/").route(web::post().to(commit::post)))
            // tables
            .service(web::resource("/api/tables/list/").route(web::get().to(table::list)))
            // table
            .service(
                web::resource("/api/table/raw/{table_name}/").route(web::get().to(table::get_raw)),
            )
            .service(
                web::resource("/api/table/raw/{table_name}/{commit_id}/")
                    .route(web::post().to(table::post_raw)),
            )
            .service(
                web::resource("/api/table/module/{table_name}/{module_name}/")
                    .route(web::delete().to(table::delete_module)),
            )
            .service(
                web::resource("/api/table/{table_name}/")
                    .route(web::get().to(table::get))
                    .route(web::post().to(table::post))
                    .route(web::delete().to(table::delete)),
            )
            .service(
                web::resource("/api/table/{table_name}/{tail:.*}")
                    .route(web::get().to(table::deep_get))
                    .route(web::post().to(table::deep_post)),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
