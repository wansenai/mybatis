use actix_web :: {get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::database::dbInfo;
use crate::service::route;
///
/// 路由网关
///

#[actix_web::main]
#[test]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/v1", web::get().to(route :: greet))
            .route("/v2/{name}", web::get().to(route :: greet))
    })
        .bind(("127.0.0.1", 8787))?
        .run().await
}

#[test]
fn getCoon(){
    dbInfo::testCoon();
}