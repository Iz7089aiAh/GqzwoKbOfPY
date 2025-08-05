use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use std::{process::Command, str};
use actix_web::http::StatusCode;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use serde::Serialize;

// Define a struct to represent a process
#[derive(Serialize)]
struct ProcessInfo {
    command: String,
    status: String,
}

// Define a struct to represent a list of processes
#[derive(Serialize)]
struct ProcessList {
    processes: Vec<ProcessInfo>,
}

// Define an error response struct
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

// Define an HTTP service handler to start a process
#[get("/start/{command}")]
async fn start_process(cmd: web::Path<String>) -> Result<HttpResponse, ErrorBadRequest> {
    let command = cmd.into_inner();
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&command)
        .output()
        .map_err(|_| ErrorBadRequest("Failed to execute command"))?;
    
    let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
    let stderr = str::from_utf8(&output.stderr).unwrap_or_default();
    let status = if output.status.success() { "success" } else { "failure" };
    Ok(HttpResponse::Ok().json(ProcessInfo {
        command,
        status: format!("stdout: {}\
stderr: {}\
status: {}", stdout, stderr, status),
    }))
}

// Define an HTTP service handler to list processes
#[get("/processes")]
async fn list_processes() -> Result<HttpResponse, ErrorInternalServerError> {
    let processes = vec![ProcessInfo {
        command: "ls".to_string(),
        status: "running".to_string(),
    }, ProcessInfo {
        command: "echo hello".to_string(),
        status: "finished".to_string(),
    }];
    Ok(HttpResponse::Ok().json(ProcessList {
        processes,
    }))
}

// Define the main function to start the Actix web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/start/{command}", get::start_process)
            .route("/processes", get::list_processes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
