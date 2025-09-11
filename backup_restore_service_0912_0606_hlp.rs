use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::time::SystemTime;

// Define a struct for the backup request data
#[derive(Serialize, Deserialize)]
struct BackupRequest {
    file_path: String,
}

// Define a struct for the restore request data
#[derive(Serialize, Deserialize)]
struct RestoreRequest {
    backup_file_path: String,
    original_file_path: String,
}

// Define the BackupRestoreService struct which contains the handlers for backup and restore operations
struct BackupRestoreService;

impl BackupRestoreService {
    // Create a backup of the file
    #[post("/backup")]
    async fn backup(&self, payload: web::Json<BackupRequest>) -> impl Responder {
        let file_path = &payload.file_path;
        let backup_file_path = format!("{}_{}.bak", file_path, SystemTime::now().elapsed_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());

        match fs::copy(&file_path, &backup_file_path) {
            Ok(_) => HttpResponse::Ok().json(json!({"message": "Backup successful", "backup_file_path": backup_file_path})),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => HttpResponse::NotFound().json(json!({"error": "File not found"})),
                _ => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
            },
        }
    }

    // Restore the file from the backup
    #[post("/restore")]
    async fn restore(&self, payload: web::Json<RestoreRequest>) -> impl Responder {
        let backup_file_path = &payload.backup_file_path;
        let original_file_path = &payload.original_file_path;

        match fs::copy(backup_file_path, original_file_path) {
            Ok(_) => HttpResponse::Ok().json(json!({"message": "Restore successful"})),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => HttpResponse::NotFound().json(json!({"error": "Backup file not found"})),
                _ => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
            },
        }
    }
}

// Define the App configuration
fn main() -> std::io::Result<()> {
    // Set up the Actix web server
    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(BackupRestoreService::backup)
            .service(BackupRestoreService::restore)
    })
    .bind("127.0.0.1:8080")?
    .run();

    // Start the server and wait for it to complete
    println!("Server running at http://127.0.0.1:8080/");
    server.await
}
