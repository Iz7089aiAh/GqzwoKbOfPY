use actix_web::{
    get,
    post,
    web,
    HttpRequest,
    HttpResponse,
    Responder,
    Error,
};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

// Define a struct to represent an in-app purchase item.
#[derive(Deserialize)]
struct PurchaseRequest {
    item_id: String,
    quantity: u32,
}

// Define a struct to represent a purchase response.
#[derive(Serialize)]
struct PurchaseResponse {
    success: bool,
    message: String,
}

// Mock database for storing in-app purchase items.
lazy_static! {
    static ref IN_APP_PURCHASE_ITEMS: HashMap<String, u32> = {
        let mut m = HashMap::new();
        m.insert("coin_pack".to_string(), 100);
        m.insert("gem_pack".to_string(), 50);
        m;
    };
}

// Define a handler for the purchase endpoint.
#[post("/purchase")]
async fn purchase(req: HttpRequest, body: web::Json<PurchaseRequest>) -> Result<impl Responder, Error> {
    // Check if the item exists in the mock database.
    let item_price = IN_APP_PURCHASE_ITEMS.get(&body.item_id).ok_or_else(|| {
        HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": format!("Item with id '{}' not found.", body.item_id),
        }))
    })?;

    // Perform the purchase logic.
    // For simplicity, this example just checks if the purchase is valid.
    if body.quantity > 0 && *item_price > 0 {
        // Simulate a successful purchase.
        Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": format!("Purchased {} of item '{}' successfully.", body.quantity, body.item_id),
        })))
    } else {
        // Return an error if the purchase is not valid.
        Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Invalid purchase.".to_string(),
        })))
    }
}

// Define the main function to setup the Actix server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the Actix web server.
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            // Add the purchase endpoint to the server.
            .service(purchase)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
