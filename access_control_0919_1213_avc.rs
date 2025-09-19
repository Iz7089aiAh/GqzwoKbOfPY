use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use actix_web::middleware::identity::CookieIdentityPolicy;
use actix_web::middleware::identity::{Identity, IdentityService};
use std::collections::HashSet;

/// Represents the user's roles
#[derive(Clone, Copy)]
enum Role {
    Admin,
    User,
    // Add more roles as needed
}

/// Simulate a database of user roles
lazy_static::lazy_static! {
    static ref USER_ROLES: HashSet<(String, Role)> = {
        let mut set = HashSet::new();
        set.insert(("admin".to_string(), Role::Admin));
        set.insert(("user".to_string(), Role::User));
        set
    };
}

/// Middleware to check user's role
async fn role_check(role: web::Path<(&'static str, Role)>) -> impl Responder {
    let user_id = role.0;
    let required_role = role.1;

    if let Some((user_id, user_role)) = USER_ROLES.get().find(|&(ref id, _)| id == user_id) {
        if user_role >= required_role {
            HttpResponse::Ok().body("Access granted")
        } else {
            HttpResponse::Forbidden().body("Access denied")
        }
    } else {
        HttpResponse::Unauthorized().body("User not found")
    }
}

/// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up a cookie policy
    let cookie_policy = CookieIdentityPolicy::new("myapp", &[]);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(cookie_policy))
            .route("/admin/{role}", web::get().to(role_check))
            .route("/user/{role}", web::get().to(role_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Add necessary dependencies in Cargo.toml
// [dependencies]
// actix-web = "4.0"
// actix-identity = "0.3"
// lazy_static = "1.4"
