use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use serde::Deserialize;
use serde_json::json;
use validator::{Validate, ValidationError, Validator};

// 定义表单数据结构
#[derive(Deserialize, Validate)]
struct FormData {
    #[validate(email)]
    email: String,

    #[validate(length(min = 3))]
    username: String,
}

// 表单数据验证函数
async fn validate_form(data: web::Json<FormData>) -> impl Responder {
    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().json(json!({ 