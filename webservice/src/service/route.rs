use actix_web :: {HttpRequest, HttpResponse, Responder};

#[allow(dead_code)]
pub async fn test () -> impl Responder {
    HttpResponse::Ok().body("test")
}

#[allow(dead_code)]
pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[allow(dead_code)]
pub async fn creat(req: HttpRequest) -> impl Responder {
    let var = req.match_info().get("var").unwrap_or("api2");
    format!("调用{}", var)
}
