use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use std::fs;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use serde_json::json;

// FolderStructure represents the structure of a folder
#[derive(Deserialize, Serialize, Debug)]
pub struct FolderStructure {
    path: String,
    files: Vec<String>,
    folders: Vec<String>,
}

// OrganizeFolderResponse is the response structure for organizing a folder
#[derive(Serialize, Debug)]
pub struct OrganizeFolderResponse {
    message: String,
}

// OrganizeFolderRequest is the request structure for organizing a folder
#[derive(Deserialize)]
pub struct OrganizeFolderRequest {
    path: String,
}

// OrganizeFolder organizes the given folder by listing all files and subfolders
#[get("/organize/{path}")]
async fn organize_folder(path: web::Path<String>) -> impl Responder {
    let folder_path = path.into_inner();
    match fs::read_dir(folder_path.as_str()) {
        Ok(entries) => {
            let mut files = Vec::new();
            let mut folders = Vec::new();

            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            folders.push(path.to_str().unwrap().to_string());
                        } else {
                            files.push(path.to_str().unwrap().to_string());
                        }
                    },
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(json!({"message": "Error reading directory"}));
                    },
                }
            }

            let structure = FolderStructure {
                path: folder_path,
                files,
                folders,
            };

            HttpResponse::Ok().json(json!(structure))
        },
        Err(_) => {
            HttpResponse::BadRequest().json(json!({"message": "Invalid path"}))
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(organize_folder)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
