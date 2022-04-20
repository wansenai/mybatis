use anyhow::Result;
use bytes::Bytes;
use lru::LruCache;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use axum::{
    extract::{Path, Extension}, 
    handler::get, 
    http::{StatusCode, HeaderMap, HeaderValue}, 
    Router,
    AddExtensionLayer,
};
use std::{
    convert::TryInto,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};
use tracing::{
    info,
    instrument,
};

// 引入protobuf 生成的代码 
mod pb;
mod engine;

use pb::*;
use image::ImageOutputFormat;
use engine::{Engine, Photon};

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}
type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("匹配缓存 {}", key);
            v.to_owned()
        }
        None => {
            info!("返回url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };

    Ok(data)
}

async fn generate(Path(Params { spec, url}): Path<Params>, Extension(cache): Extension<Cache>) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url: &str = &percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let data = retrieve_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // TODO: 处理图片
    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    engine.apply(&spec.specs);

    let image = engine.generate(ImageOutputFormat::Jpeg(85));

    info!("以完成图片处理：图片size {}", image.len());
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("images/jpeg"));

    Ok((headers, image))
}

#[tokio::main]
async fn main() {
    // 初始化tracing
    tracing_subscriber::fmt::init();
    let cache: Cache =Arc::new(Mutex::new(LruCache::new(1024)));

    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );

    let addr = "127.0.0.1:3000".parse().unwrap();

    print_test_url("https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260");

    info!("监听地址:{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();  
}

// debug 辅助函数
fn print_test_url(url: &str){
    use std::borrow::Borrow;

    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url : http://localhost:3000/image/{}/{}", s, test_image)
}