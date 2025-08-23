use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use actix_web::middleware::Logger;
use std::collections::HashSet;
use regex::Regex;

/// A simple service that demonstrates basic XSS protection.
/// It filters incoming text inputs to prevent XSS attacks by escaping HTML special characters.
#[get("/xss_protect")]
async fn xss_protection() -> impl Responder {
    // Example of an input that might be received from a user
    let user_input = "<script>alert('XSS')</script>";

    // Escape HTML special characters to prevent XSS attacks
    let safe_input = escape_html(user_input);
# 添加错误处理

    // Return the sanitized input as a response
# NOTE: 重要实现细节
    HttpResponse::Ok().body(safe_input)
}
# NOTE: 重要实现细节

/// Escapes HTML special characters in a string to prevent XSS attacks.
fn escape_html(input: &str) -> String {
    let mut escaped = String::new();
    let html_special_chars = vec!['&', '<', '>', '"', '\''];
    let html_special_chars_escaped = vec!["\u0026", "\u003c", "\u003e", "\u0022", "\u0027"];

    for c in input.chars() {
        if let Some(index) = html_special_chars.iter().position(|&x| x == c) {
            escaped.push_str(html_special_chars_escaped[index]);
# TODO: 优化性能
        } else {
            escaped.push(c);
        }
    }

    escaped
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
# TODO: 优化性能
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
# 优化算法效率
            .service(xss_protection)
# FIXME: 处理边界情况
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// A HashSet for quick lookup of HTML special characters that are potentially dangerous.
lazy_static::lazy_static! {
    static ref DANGEROUS_CHARS: HashSet<char> = {
        let mut s = HashSet::new();
        s.insert('&');
        s.insert('<');
        s.insert('>');
        s.insert('"');
        s.insert('\'');
        s
    };
}

/// Uses a regular expression to check if a string contains any potentially dangerous characters.
# 改进用户体验
/// If it does, it returns an error.
fn check_for_dangerous_chars(input: &str) -> Result<(), String> {
    let re = Regex::new(r"[&<>"']").unwrap();
# NOTE: 重要实现细节
    if re.is_match(input) {
        Err("Input contains potentially dangerous characters.".to_string())
    } else {
        Ok(())
    }
# 扩展功能模块
}

/// This function combines the escaping and checking functions to provide a comprehensive
/// protection against XSS attacks.
async fn protected_input(input: web::Json<String>) -> Result<HttpResponse, actix_web::Error> {
    let input_str = input.into_inner();

    // Check for potentially dangerous characters
    check_for_dangerous_chars(&input_str).map_err(|e| {
# FIXME: 处理边界情况
        HttpResponse::BadRequest().body(e)
# 扩展功能模块
    })?;

    // Escape HTML special characters
    let safe_input = escape_html(&input_str);
# 扩展功能模块

    Ok(HttpResponse::Ok().body(safe_input))
}
