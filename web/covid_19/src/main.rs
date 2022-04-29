mod domain;
mod common;
mod api;
mod service;

use std::io::Result;
use actix_cors::Cors;
use actix_web::{
    web::{self},
    App,
    HttpServer,
    middleware::Logger,
    http::header,
};

use api::{
    user_api,
    nucleic_api,
};

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600),
        )
        .wrap(Logger::default())
        .service(
            web::scope("/api")
            .service(user_api::user_register)
            .service(user_api::user_login)
            .service(nucleic_api::nucleic_register)
            .service(nucleic_api::institution_register)
        )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}