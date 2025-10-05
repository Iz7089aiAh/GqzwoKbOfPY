use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Result};

/// Represents a discount code and the corresponding percentage discount.
struct Discount {
    /// The discount code as a String.
    code: String,
    /// The discount percentage as a u8.
    percentage: u8,
}

/// Handles GET requests to apply a discount to a given amount.
///
/// # Arguments
/// * `code` - A string slice representing the discount code.
/// * `amount` - A f64 representing the original price.
#[get("/apply_discount/{code}/{amount}")]
async fn apply_discount(code: web::Path<(String, f64)>) -> Result<impl Responder> {
    let (discount_code, amount) = code.into_inner();
    let discount_percentage = match get_discount_percentage(&discount_code) {
        Some(percentage) => percentage,
        None => return Ok(HttpResponse::BadRequest().finish()),
    };

    let discount_amount = calculate_discount(amount, discount_percentage);
    Ok(HttpResponse::Ok().json(discount_amount))
}

/// Retrieves the discount percentage for a given discount code.
///
/// # Arguments
/// * `code` - A string slice representing the discount code.
///
/// # Returns
/// * An Option<u8> where Some(u8) contains the discount percentage if the code is valid,
///   or None if the code is invalid.
fn get_discount_percentage(code: &str) -> Option<u8> {
    // Here you would implement the logic to check the discount code against a database or
    // some other data source. For simplicity, this example uses a hardcoded map.
    let valid_codes = vec![("DISCOUNT10", 10), ("DISCOUNT20", 20)];
    valid_codes.iter()
        .find(|&&(ref c, _)| c == code)
        .map(|&(_, percentage)| percentage)
}

/// Calculates the discount amount based on the original price and the discount percentage.
///
/// # Arguments
/// * `amount` - A f64 representing the original price.
/// * `percentage` - A u8 representing the discount percentage.
///
/// # Returns
/// * A f64 representing the discount amount.
fn calculate_discount(amount: f64, percentage: u8) -> f64 {
    amount * (percentage as f64) / 100.0
}

/// Runs the Actix web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(apply_discount)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
