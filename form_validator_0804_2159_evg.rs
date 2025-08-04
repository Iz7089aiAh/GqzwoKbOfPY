use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_validator::Validate;
# TODO: 优化性能
use serde_json::{json, Value};

// 定义一个结构体，用于反序列化和验证表单数据
#[derive(Serialize, Deserialize, Validate)]
pub struct FormData {
    #[validate(length(min = 3, max = 100))]
    username: String,

    #[validate(range(min = 1, max = 150))]
    age: u32,
}

// 异步处理POST请求的函数
#[post("/form")]
async fn form_validator(form_data: web::Json<FormData>) -> impl Responder {
    // 验证表单数据
    if let Err(e) = form_data.validate() {
        // 如果验证失败，返回错误信息
        return HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
# 增强安全性
        }))
    }
    
    // 如果验证成功，返回验证通过的消息
# TODO: 优化性能
    HttpResponse::Ok().json(json!({
        "message": "Validation successful"
    }))
}

// 主函数，设置服务器和路由
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(form_validator)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 测试用的表单数据
#[cfg(test)]
mod tests {
    use super::*;
# 改进用户体验
    use actix_web::{test, web, App};
    use serde_json::json;
    
    #[actix_web::test]
# NOTE: 重要实现细节
    async fn test_form_validator_success() {
        let app = test::init_service(App::new()
            .service(form_validator))
            .await;
        
        let req = test::TestRequest::post()
# NOTE: 重要实现细节
            .uri("/form")
            .set_payload(json!({
                "username": "JohnDoe",
                "age": 30
            })).to_request();
# TODO: 优化性能
        
        let resp = app.call(req).await.unwrap();
        assert!(resp.status().is_success());
# 增强安全性
    }

    #[actix_web::test]
    async fn test_form_validator_failure() {
        let app = test::init_service(App::new()
# 扩展功能模块
            .service(form_validator))
            .await;
        
        let req = test::TestRequest::post()
            .uri("/form")
            .set_payload(json!({
                "username": "JD",
                "age": 300
            })).to_request();
        
        let resp = app.call(req).await.unwrap();
        assert!(resp.status().is_bad_request());
    }
}
