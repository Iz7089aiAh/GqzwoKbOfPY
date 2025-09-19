use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post, Responder};
use actix_files as fs;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;

/// Main handler for the batch rename tool
/// This handler accepts a list of files and their new names
/// and performs the renaming operation.
#[post("/rename")]
async fn rename_files(files: web::Json<Vec<RenameRequest>>) -> impl Responder {
    let mut results = Vec::new();

    for file in files {
        match rename_file(&file.old_name, &file.new_name) {
            Ok(_) => results.push(RenameResult {
                original: file.old_name,
                new_name: file.new_name,
                status: "success".to_string(),
            }),
            Err(e) => results.push(RenameResult {
                original: file.old_name,
                new_name: file.new_name,
                status: e.to_string(),
            }),
        }
    }

    HttpResponse::Ok().json(results)
}

/// Renames a single file
fn rename_file(old_name: &str, new_name: &str) -> io::Result<()> {
    let old_path = PathBuf::from(old_name);
    let new_path = PathBuf::from(new_name);

    // Check if the old file exists before attempting to rename
    if !old_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    // Check if the new file name already exists
    if new_path.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "File already exists"));
    }

    fs::rename(old_path, new_path)
}

/// Structure to hold the request for renaming a file
struct RenameRequest {
    old_name: String,
    new_name: String,
}

/// Structure to hold the result of renaming a file
struct RenameResult {
    original: String,
    new_name: String,
    status: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Batch Rename Tool")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(rename_files)
            // Serve static files from the `public` directory
            .service(fs::Files::new(