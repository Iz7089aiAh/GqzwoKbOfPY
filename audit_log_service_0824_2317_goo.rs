use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use std::sync::Mutex;
use lazy_static::lazy_static;
use log::{info, warn, error};

// A struct to hold the audit logs
struct AuditLog {
    entries: Mutex<Vec<String>>,
}

// A struct to represent a single audit log entry
struct LogEntry {
    timestamp: String,
    message: String,
}

impl AuditLog {
    // New function to create a new AuditLog instance
    fn new() -> Self {
        AuditLog {
            entries: Mutex::new(Vec::new()),
        }
    }

    // Function to add a new entry to the audit log
    fn log(&self, log_entry: LogEntry) {
        let mut entries = self.entries.lock().unwrap();
        entries.push(format!("[{}] {}", log_entry.timestamp, log_entry.message));
    }
}

// Function to handle GET requests for the audit log
#[get("/audit_log")]
async fn get_audit_log() -> impl Responder {
    let audit_log = web::Data::get::<web::Data<AuditLog>>().unwrap();
    let entries = audit_log.entries.lock().unwrap();
    HttpResponse::Ok().body(entries.join("\
"))
}

// Main function to setup the Actix web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the audit log
    let audit_log = AuditLog::new();

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(audit_log.clone()))
            .service(get_audit_log)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Lazy static for logging purposes
lazy_static! {
    static ref AUDIT_LOG: AuditLog = AuditLog::new();
}

// Function to log an event with a timestamp
fn log_event(message: &str) {
    let log_entry = LogEntry {
        timestamp: chrono::Local::now().to_rfc3339(),
        message: message.to_string(),
    };
    AUDIT_LOG.log(log_entry);
}

// Example usage of log_event
#[allow(dead_code)]
fn example_usage() {
    log_event("This is a test log entry.");
}