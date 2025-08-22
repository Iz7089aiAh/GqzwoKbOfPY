use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::fs;
use std::io::Error;
use std::env;
use std::process::Command;
use std::time::SystemTime;

// Define a structure to hold the backup file details
#[derive(Serialize, Deserialize)]
struct BackupFile {
    filename: String,
    timestamp: u64,
}

// Define a structure to hold the restore request details
#[derive(Serialize, Deserialize)]
struct RestoreRequest {
    filename: String,
}

// Define a service handler for backup files
async fn backup_file(file_path: web::Path<String>) -> impl Responder {
    let path = PathBuf::from(file_path.into_inner());

    // Check if file exists
    if !path.exists() {
        return HttpResponse::NotFound().body("File not found");
    }

    // Create a timestamp for the backup file
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Create a backup filename with the timestamp
    let backup_filename = format!("{}_{}