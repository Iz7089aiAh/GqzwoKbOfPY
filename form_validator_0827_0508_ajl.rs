use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use serde::Deserialize;
use serde_json::json;
use validator::{Validate, ValidationError, Validator};

// 定义一个表单数据结构体
#[derive(Deserialize, Validate)]
struct FormData {
    #[validate(length(min = 1, max = 100))]
    name: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(custom = "validate_password", custom_message = "Password must be at least 8 characters long", len = "8..128"))]
    password: String,
}

// 自定义密码验证函数
fn validate_password(field: &str) -> bool {
    field.len() >= 8 && field.len() <= 128
}

// 表单数据验证器
async fn validate_form(data: web::Json<FormData>) -> impl Responder {
    let mut errors = Vec::new();
    
    // 验证表单数据
    if let Err(e) = data.validate() {
        for err in e.errors {
            errors.push(json!({
                