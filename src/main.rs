#![allow(dead_code)]

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

mod error;
mod modules;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,debug");
    env_logger::init();

    log::info!("Starting HTTP server on 0.0.0.0:80...");
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new().wrap(cors).wrap(Logger::default())
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
