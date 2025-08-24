use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::error::Error;
use serde::Serialize;
use serde_json::json;
use actix_files as fs;
use actix_web::middleware::Logger;

// Define a custom error type for our application
#[derive(Debug, Serialize)]
enum BackupSyncError {
    FileNotFound(PathBuf),
    IOError(io::Error),
}

impl From<io::Error> for BackupSyncError {
    fn from(err: io::Error) -> Self {
        BackupSyncError::IOError(err)
    }
# FIXME: 处理边界情况
}

// Define a struct to handle file backup and synchronization
struct FileBackupSync;

impl FileBackupSync {
    // Synchronize a file from source to destination
    fn sync_file(&self, src: &Path, dst: &Path) -> Result<(), BackupSyncError> {
        if !src.exists() {
            return Err(BackupSyncError::FileNotFound(src.to_path_buf()));
        }
        let mut src_file = File::open(src)?;
# 改进用户体验
        let mut dst_file = File::create(dst)?;
        io::copy(&mut src_file, &mut dst_file)?;
        Ok(())
    }
}

// Define a handler for the backup endpoint
async fn backup_file(req: HttpRequest) -> impl Responder {
    let src = req.match_info().get("src").unwrap();
# 改进用户体验
    let dst = req.match_info().get("dst").unwrap();
    let path_src = Path::new(src);
    let path_dst = Path::new(dst);

    match FileBackupSync::new().sync_file(&path_src, &path_dst) {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"}