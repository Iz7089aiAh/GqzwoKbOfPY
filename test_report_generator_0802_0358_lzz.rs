 * and is structured for maintainability and extensibility.
 */

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::json;

// Define a struct to represent the input data for the test report.
#[derive(Deserialize)]
struct ReportInput {
    tests: Vec<(String, String)>, // A vector of tuples containing test name and outcome
}

// Define a struct to represent a generated test report.
struct TestReport {
    report_data: HashMap<String, String>,
}

impl TestReport {
    // Constructor for TestReport
    fn new() -> Self {
        TestReport {
            report_data: HashMap::new(),
        }
    }

    // Method to generate the report data based on input data.
    fn generate_report(&mut self, input: &ReportInput) -> Result<(), String> {
        for (test_name, outcome) in &input.tests {
            self.report_data.insert(test_name.clone(), outcome.clone());
        }
        Ok(())
    }
}

// Handler for generating test reports
#[get("/report")]
async fn generate_report_handler() -> impl Responder {
    // Mock input data for demonstration purposes.
    let mock_input = ReportInput {
        tests: vec![(
            "Test 1".to_string(),
            "Passed".to_string(),
        ), (
            "Test 2".to_string(),
            "Failed".to_string(),
        )],
    };

    // Create a new TestReport instance.
    let mut report = TestReport::new();

    // Attempt to generate the report.
    if let Err(e) = report.generate_report(&mock_input) {
        return HttpResponse::InternalServerError().body(e);
    }

    // Return the generated report data as a JSON response.
    let report_json = json!(report.report_data).to_string();
    HttpResponse::Ok().body(report_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server with the report generation handler.
    HttpServer::new(|| {
        App::new()
            .service(generate_report_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
