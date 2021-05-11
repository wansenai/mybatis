use actix_web :: {HttpRequest, HttpResponse, Responder};

pub async fn test () -> impl Responder {
    HttpResponse::Ok().body("test")
}

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub async fn craat(req: HttpRequest) -> impl Responder {
    let var = req.match_info().get("var").unwrap_or("api2");
    format!("调用{}", var)
}
