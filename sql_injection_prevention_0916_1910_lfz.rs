// sql_injection_prevention.rs
// This program demonstrates how to prevent SQL injection using Rust and Actix framework.

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use actix_web::web::Json;

// Define a struct to represent our database connection pool.
struct DatabasePool(
    r2d2::Pool<diesel::pg::PgConnection>,
);

// Define a struct to handle requests.
struct SearchQuery {
    query: String,
}

// Implement content negotiation for our SearchQuery struct to accept JSON input.
impl actix_web::FromRequest for SearchQuery {
    type Error = actix_web::Error;
    type Config = ();
    type Future = actix_web::web::Either<
        actix_web::web::JsonFuture<SearchQuery>,
        actix_web::web::FutureResponse<actix_web::Error>,
    >;
    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::web::Payload) -> Self::Future {
        actix_web::web::Either::Left(
            actix_web::web::JsonFuture::new().from_request(req, payload)
        )
    }
}

#[get("/search")]
async fn search_items(pool: web::Data<DatabasePool>, query: web::Json<SearchQuery>) -> impl Responder {
    // Get a connection from the pool.
    let conn = pool.get().expect("Failed to get a database connection from the pool.");

    // Prevent SQL injection by using parameterized queries.
    let items = diesel::sql_query("SELECT * FROM items WHERE name = $1")
        .bind::<String, _>(&query.query)
        .load::<Item>(&conn)
        .expect("Error loading items from database");

    // Return the items as JSON.
    HttpResponse::Ok().json(items)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a connection pool for the database.
    let database_url = "postgres://username:password@localhost/database_name";
    let manager = r2d2_diesel::postgres::PgConnectionManager::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a database pool.");

    // Define the application state.
    let database_pool = web::Data::new(DatabasePool(pool));

    // Start the HTTP server.
    HttpServer::new(move || {
        App::new()
            .app_data(database_pool.clone())
            .service(search_items)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Assume we have a table called `items` with a column named `name`.
#[derive(Queryable)]
struct Item {
    id: i32,
    name: String,
}

// This is a placeholder for the actual database schema.
mod schema {
    diesel::table! {
        items (id) {
            id -> Integer,
            name -> Text,
        }
    }
}

// Import necessary modules for Diesel.
mod models;
mod schema;

// Use Diesel's macros to implement the necessary logic.
use self::models::Item;
use self::schema::items::dsl::*;

// Add necessary Diesel traits.
use diesel::*;
use diesel::pg::Pg;
use diesel::r2d2::{ConnectionManager, Pool};

// Define the module for database models.
pub mod models {
    pub use super::schema::*;
}

// Define the module for database schema.
pub mod schema;

// Define a module for database connection pool.
pub mod pool;

// Add necessary Diesel traits and structs.
pub use diesel::*;
pub use diesel::pg::PgConnection;
pub use diesel::r2d2::{ConnectionManager, Pool};

// Add necessary Actix Web traits and structs.
pub use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
pub use actix_web::web::Json;
