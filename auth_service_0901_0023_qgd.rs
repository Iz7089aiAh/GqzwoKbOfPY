use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Result};
use serde::{Deserialize, Serialize};
use actix_web::http::StatusCode;
use actix_web::error::ErrorBadRequest;

// 用户身份数据结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

// 用户认证请求数据结构
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

// 用户认证响应数据结构
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub authenticated: bool,
    pub user: Option<User>,
}

// 用户认证服务
pub async fn authenticate_user(auth_request: web::Json<AuthRequest>) -> Result<HttpResponse, ErrorBadRequest> {
    // 假设的验证逻辑
    let user = authenticate(&auth_request.username, &auth_request.password).await;
    match user {
        Some(u) => Ok(HttpResponse::Ok().json(AuthResponse {
            authenticated: true,
            user: Some(u),
        })),
        None => Ok(HttpResponse::Ok().json(AuthResponse {
            authenticated: false,
            user: None,
        })),
    }
}

// 模拟的用户数据库
async fn authenticate(username: &str, password: &str) -> Option<User> {
    // 假设的用户信息
    let users = vec![User {
        username: "admin".to_string(),
        password: "password123".to_string(),
    }];

    // 简单的验证逻辑
    users.iter().find(|&u| u.username == username && u.password == password).cloned()
}

#[get("/authenticate")]
async fn auth_route() -> impl Responder {
    // 这里应该有一个请求体解析为AuthRequest的步骤，但为了简化，我们直接返回一个示例请求
    let auth_request = AuthRequest {
        username: "admin".to_string(),
        password: "password123".to_string(),
    };

    // 调用用户认证服务
    authenticate_user(web::Json::from(auth_request)).await
}

// 定义应用程序
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(auth_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 错误处理
impl actix_web::error::ResponseError for ErrorBadRequest {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
