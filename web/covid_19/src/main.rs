mod domain;
mod common;
mod api;
mod service;

use std::io::Result;
use actix_web::{
    web::{self},
    App,
    HttpServer,
};

use api::{
    user_api,
    nucleic_api,
};

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
            .service(user_api::user_register)
            .service(nucleic_api::nucleic_register)
        )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}