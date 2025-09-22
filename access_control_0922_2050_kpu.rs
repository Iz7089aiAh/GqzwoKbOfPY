use actix_web::{get, HttpResponse, Responder, web};
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use actix_web::http::StatusCode;
use serde::Deserialize;

// 定义用户身份信息
#[derive(Deserialize)]
struct UserAuth {
    token: String,
}

// 模拟用户的权限级别
enum PermissionLevel {
    Admin,
    User,
    Guest,
}

// 访问控制逻辑
impl PermissionLevel {
    fn check_permission(&self, level: PermissionLevel) -> bool {
        matches!(self, PermissionLevel::Admin) || matches!(self, level)
    }
}

// 访问控制中间件
async fn auth_middleware(req: actix_web::dev::ServiceRequest, next: actix_web::dev::ServiceResponse) -> actix_web::dev::ServiceResponse {
    let auth_header = req.headers().get("Authorization").unwrap_or("").to_str().unwrap_or("");
    // 模拟检查 token
    if !auth_header.starts_with("Bearer ") {
        return next.call(req).await; // 没有 Authorization 头，直接转发请求
    }
    
    // 解析 token，这里简化为固定值
    let token = auth_header.trim_start_matches("Bearer ").to_string();
    if token != "valid_token" {
        return Ok::<_, Error>(HttpResponse::Unauthorized().finish());
    }

    next.call(req).await
}

#[get("/secure")]
async fn secure_route(user: web::Json<UserAuth>) -> impl Responder {
    // 检查权限
    let permission_level = PermissionLevel::User;
    if !permission_level.check_permission(PermissionLevel::User) {
        return HttpResponse::Forbidden().body("Access Denied");
    }

    HttpResponse::Ok().body("Welcome to the secure area!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置中间件
    let app = actix_web::App::new()
        .wrap(auth_middleware)
        .service(secure_route);

    // 启动服务器
    actix_web::HttpServer::new(|| app.clone()).bind("127.0.0.1:8080")?.run().await
}
