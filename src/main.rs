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
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 10)) // 10mb.... really?
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(index)
            .service(dump)
            .service(restore)
            .service(nuclear)
            .service(module::get)
            .service(module::post)

            .service(hashes::remove_module)
            .service(hashes::get)
            .service(hashes::post)

            .service(commit::update_tree)
            .service(commit::get)
            .service(commit::post)

            .service(table::list)
            .service(table::get_raw) 
            .service(table::post_raw)
            .service(table::delete_module)
            .service(table::get)
            .service(table::post)
            .service(table::delete)
            .service(table::deep_get)
            .service(table::deep_post)
            .service(table::deep_delete)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}


