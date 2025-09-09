use actix_web::{web, App, HttpServer, Responder, error::ErrorBadRequest, HttpRequest, HttpResponse, Error, get, post, put, delete};
use actix_web::test::{self, TestServer};
use serde::{Serialize, Deserialize};
use serde_json::json;

// 定义一个结构体来表示我们的用户
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 定义一个简单的用户服务
struct UserService;

// 实现UserService的方法
impl UserService {
    // 获取用户信息
    #[get("/user/{id}")]
    async fn get_user(&self, web::Path(id): web::Path<u32>) -> Result<impl Responder, ErrorBadRequest> {
        // 模拟数据库中的用户数据
        let users = vec![User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
        }];

        // 查找用户
        if let Some(user) = users.into_iter().find(|u| u.id == id) {
            Ok(user)
        } else {
            // 如果用户不存在，返回错误
            Err(ErrorBadRequest::new("User not found"))?
        }
    }
}

// 集成测试函数
#[cfg(test)]
mod tests {
    use super::*;
    use test::TestServer;

    #[actix_web::test]
    async fn test_get_user() {
        let server = test::init_service(App::new()
            .configure(setup_service))
            .await;

        // 发送请求并验证响应
        let req = test::TestRequest::with_uri("/user/1").to_request();
        let resp = test::call_service(&server, req).await;
        assert!(resp.status().is_success());
        assert!(!resp.response().body().is_empty());
    }

    async fn setup_service(cfg: &mut web::ServiceConfig) {
        cfg.service(UserService::get_user);
    }
}

// 设置HTTP服务
fn setup_service(cfg: &mut web::ServiceConfig) {
    cfg.service(UserService::get_user);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(setup_service)
    }).
    listen("127.0.0.1:8080")?.run().await?;
    Ok(())
}
