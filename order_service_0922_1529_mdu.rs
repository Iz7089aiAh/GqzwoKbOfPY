use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::sync::Arc;

// Define a struct to represent an Order
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Order {
    id: u32,
    product_id: u32,
    quantity: u32,
    status: String,
}

// In-memory storage for orders
lazy_static::lazy_static! {
    static ref ORDERS: Mutex<Vec<Order>> = Mutex::new(Vec::new());
}

// Define error types
#[derive(Debug)]
enum OrderError {
    NotFound,
    InvalidData,
}

impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            OrderError::NotFound => write!(f, "Order not found"),
            OrderError::InvalidData => write!(f, "Invalid order data provided"),
        }
    }
}

impl actix_web::error::ResponseError for OrderError {
    fn error_response(&self) -> actix_web::error::Error {
        use actix_web::http::StatusCode;
        match *self {
            OrderError::NotFound =>
                actix_web::error::Error::from((StatusCode::NOT_FOUND, "Order not found")),
            OrderError::InvalidData =>
                actix_web::error::Error::from((StatusCode::BAD_REQUEST, "Invalid order data")),
        }
    }
}

// Handler for creating a new order
async fn create_order(order: web::Json<Order>) -> Result<HttpResponse, OrderError> {
    // Check for valid data
    if order.quantity == 0 {
        return Err(OrderError::InvalidData);
    }

    // Add order to in-memory storage
    let mut orders = ORDERS.lock().unwrap();
    orders.push(order.into_inner());

    Ok(HttpResponse::Created().json({"id": order.id}