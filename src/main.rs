#![allow(dead_code)]

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

use crate::{modules::product, utils::database::PostgresRepository};

mod error;
mod modules;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,debug");
    env_logger::init();

    let repo = Arc::new(PostgresRepository::new().await);

    log::info!("Starting HTTP server on 0.0.0.0:80...");
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new().wrap(cors).wrap(Logger::default())
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
