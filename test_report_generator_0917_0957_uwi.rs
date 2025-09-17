use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder};
use serde::Serialize;

/// Define a struct for the test report data.
#[derive(Serialize)]
struct TestReport {
    /// The title of the report
    title: String,
    /// The summary of the test results
    summary: String,
    /// The detailed results
    details: String,
}

/// Define a handler function to generate the test report.
#[get("/report")]
async fn generate_report() -> impl Responder {
    // Define a sample test report
    let report_data = TestReport {
        title: "Test Report".into(),
        summary: "This is a summary of test results.".into(),
        details: "Details of the test results".into(),
    };

    // Serialize the report data into JSON
    let report_json = serde_json::to_string(&report_data).expect("Failed to serialize report data");

    // Return the serialized JSON as a response
    HttpResponse::Ok().content_type("application/json").body(report_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server with the report generator endpoint
    HttpServer::new(|| {
        App::new()
            .service(generate_report)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}