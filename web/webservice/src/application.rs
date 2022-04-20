use actix_web :: {web, App, HttpServer,};
use crate::database::db_info;
use crate::service::route;
///
/// 路由网关
///

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/v1", web::get().to(route :: greet))
            .route("/v2/{name}", web::get().to(route :: creat))
    })
        .bind(("127.0.0.1", 8787))?
        .run().await
}

#[allow(dead_code)]
fn get_coon(){
    db_info::test_coon();
}