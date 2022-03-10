use axum::{extract::Path, handler::get, http::StatusCode, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::convert::TryInto;

// 引入protobuf 生成的代码 
mod pb;

use pb::*;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

async fn generate(Path(Params { spec, url}): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url : {}\n spec: {:#?}", url, spec))
}

#[tokio::main]
async fn main() {
    // 初始化tracing
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/image/:spec/:url", get(generate));

    let addr = "127.0.0.1:3002".parse().unwrap();
    tracing::debug!("监听地址:{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();  
}
