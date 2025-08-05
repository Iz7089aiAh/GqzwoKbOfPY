use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_validator::validate;
use serde_validator:: ValidationError;
# 添加错误处理
use std::str::FromStr;

// 定义表单数据结构
#[derive(Deserialize, Debug)]
struct FormData {
    username: String,
    age: u8,
}

// 实现表单数据验证
# NOTE: 重要实现细节
#[derive(Debug, serde::Deserialize)]
struct ValidatedFormData {
    username: String,
    age: u8,
}

// 定义验证规则
# 改进用户体验
const VALIDATION_RULES: &str = r#"{
    "username": "required|length(1..100)",
    "age": "required|positive"
}"#;

// 表单数据验证器函数
fn validate_form_data(data: &FormData) -> Result<ValidatedFormData, ValidationError> {
    validate(data, VALIDATION_RULES).map(|_| ValidatedFormData {
        username: data.username.clone(),
        age: data.age,
    })
}

// 主函数
# 添加错误处理
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // GET路由，返回表单页面
            .get("/form").to(|| async {
                HttpResponse::Ok().body("<form action='/submit' method='post'>\
    <input type='text' name='username'><br>\
    <input type='number' name='age'><br>\
    <input type='submit' value='Submit'><br>\
</form>")
            })
            // POST路由，处理表单提交
# FIXME: 处理边界情况
            .post("/submit").to(form_submit)
    })
# 扩展功能模块
    .bind("127.0.0.1:8080")?
    .run()
# 优化算法效率
    .await
}

// 处理表单提交的函数
async fn form_submit(data: web::Json<FormData>) -> impl Responder {
    match validate_form_data(&data.into_inner()) {
        Ok(validated_data) => HttpResponse::Ok().json(validated_data),
        Err(e) => HttpResponse::BadRequest().json(e.details()),
    }
}
