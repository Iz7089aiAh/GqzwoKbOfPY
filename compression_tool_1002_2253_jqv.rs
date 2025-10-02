use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use flate2::{write::GzEncoder, read::GzDecoder, Compression};
use std::io::{self, Read, Write};

/// 压缩数据并返回压缩后的字节流
#[get("/compress/{data}")]
async fn compress_data(data: web::Path<String>) -> impl Responder {
    let data = data.into_inner();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    if let Err(e) = encoder.write_all(data.as_bytes()) {
        return HttpResponse::InternalServerError().body(format!("Compression error: {}", e));
    }
    let compressed_data = encoder.finish().unwrap();
    HttpResponse::Ok().body(compressed_data)
}

/// 解压数据并返回解压后的文本
#[get("/decompress/{data}")]
async fn decompress_data(data: web::Path<(Vec<u8>,)>) -> impl Responder {
    let data = data.into_inner().0;
    let mut decompressor = GzDecoder::new(&data[..]);
    let mut decompressed_data = String::new();
    if let Err(e) = decompressor.read_to_string(&mut decompressed_data) {
        return HttpResponse::InternalServerError().body(format!("Decompression error: {}", e));
    }
    HttpResponse::Ok().body(decompressed_data)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(compress_data)
            .service(decompress_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
