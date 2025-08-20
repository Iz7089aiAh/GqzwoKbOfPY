use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

/// Handles a POST request to uncompress a file.
/// Accepts a GZip-encoded file and writes the uncompressed data to a file.
async fn uncompress_file() -> impl Responder {
    let content = web::Bytes::from_request().expect("Error getting request body");
    // Create a variable to hold the decompressed data
    let mut decompressed_data = Vec::new();
    // Create a GZip decoder and buffer to read the content
    let file = GzDecoder::new(&*content);
    let mut reader = BufReader::new(file);
    
    // Read the decompressed data into the buffer
    reader.read_to_end(&mut decompressed_data).expect("Failed to decompress file");
    
    // Specify the output file path
    let output_path = "./uncompressed_file";
    
    // Write the decompressed data to the output file
    fs::write(output_path, &decompressed_data).expect("Failed to write to file");
    
    HttpResponse::Ok().body("File uncompressed successfully!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server at localhost:8080
    HttpServer::new(|| {
        App::new().route("/uncompress", web::post().to(uncompress_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}