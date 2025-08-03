// access_control.rs
// 使用 Rust 和 Actix 框架实现访问权限控制的示例程序。

use actix_web::{
    get,
    web,
    HttpResponse,
    Responder,
    Error,
    App,
    HttpServer,
};
use actix_web::middleware::Logger;
use actix_web::dev::ServiceRequest;
use actix_web::middleware::identity::RequestIdentity;
use std::sync::Mutex;
use std::collections::HashMap;

// 模拟的用户数据库
lazy_static::lazy_static! {
    static ref USERS: Mutex<HashMap<String, String>> = Mutex::new(
        HashMap::from([
            ("alice".to_string(), "password123".to_string()),
            ("bob".to_string(), "password456".to_string()),
        ])
    );
}

// 定义一个错误类型
#[derive(Debug)]
enum AuthError {
    Unauthorized,
    NotFound,
}

impl actix_web::error::ResponseError for AuthError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match *self {
            AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
            AuthError::NotFound => HttpResponse::NotFound().finish(),
        }
    }
}

// 从请求中提取用户名和密码
async fn extract_credentials(req: &mut ServiceRequest) -> Result<(String, String), AuthError> {
    let user_identities = req.identity();
    if let Some(user_identity) = user_identities.get("user") {
        let user_id = user_identity.get().await;
        if let Some(user) = user_id.as_ref().map(String::as_str) {
            Ok((user.to_string(), "".to_string())) // 密码目前不使用
        } else {
            Err(AuthError::Unauthorized)
        }
    } else {
        Err(AuthError::Unauthorized)
    }
}

// 验证用户凭据
async fn authenticate(credentials: (String, String)) -> Result<String, AuthError> {
    let users = USERS.lock().unwrap();
    if users.contains_key(&credentials.0) && users.get(&credentials.0).unwrap() == &credentials.1 {
        Ok(credentials.0)
    } else {
        Err(AuthError::Unauthorized)
    }
}

#[get("/secure")]
async fn secure_endpoint(credentials: web::Query<(String, String)>) -> Result<HttpResponse, Error> {
    let credentials = extract_credentials(credentials.0).await?;
    authenticate(credentials).await.map_err(|_| AuthError::Unauthorized)?;
    Ok(HttpResponse::Ok().body("Welcome to the secure page!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestIdentity::new(
                RequestIdentityConfig::default()
                    .secure()
                    .timeout(30),
            ))
            .service(secure_endpoint)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
