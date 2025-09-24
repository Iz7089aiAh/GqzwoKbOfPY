 * follows Rust best practices for maintainability and scalability.
 */

use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, Error};
use actix_web::test::{self, TestServer};
use std::collections::HashMap;

// Define a simple struct to simulate a data model
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

// Define a service error
#[derive(Debug)]
enum ServiceError {
    NotFound,
    InvalidInput,
}

// Implement `Responder` for `ServiceError`
impl actix_web::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::NotFound => HttpResponse::NotFound()
                .json(ServiceError::NotFound),
            ServiceError::InvalidInput => HttpResponse::BadRequest()
                .json(ServiceError::InvalidInput),
        }
    }
}

// Define an asynchronous handler function
async fn user_info(req: HttpRequest) -> Result<impl Responder, Error> {
    // Simulate user data retrieval
    let user_id = req.match_info().get("id").unwrap().parse::<u32>().map_err(|_| ServiceError::InvalidInput)?;
    let user = User {
        id: user_id,
        name: "Test User".to_string(),
    };

    Ok(HttpResponse::Ok().json(user))
}

// Define a test function
#[actix_web::test]
async fn test_user_info() {
    let mut srv = TestServer::new(|| {
        App::new()
            .route("/user/{id}", web::get().to(user_info))
    });

    // Test the response when a valid user ID is provided
    let res = srv.get("/user/1").send().await.unwrap();
    assert!(res.status().is_success());

    // Test the response when an invalid user ID is provided
    let res = srv.get("/user/invalid").send().await.unwrap();
    assert!(!res.status().is_success());
}

#[actix_web::test]
async fn test_user_not_found() {
    let srv = TestServer::new(|| {
        App::new()
            .route("/user/{id}", web::get().to(user_info))
    });

    let res = srv.get("/user/999").send().await.unwrap();
    assert_eq!(res.status(), actix_web::http::StatusCode::NOT_FOUND);
}

fn main() {
    // Run the tests
    test_user_info().await;
    test_user_not_found().await;
}
