use actix_web::{web, App, HttpServer, Responder, get, post, HttpResponse};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
# 扩展功能模块

#[derive(Serialize, Deserialize)]
# TODO: 优化性能
struct PasswordRequest {
# 优化算法效率
    password: String,
}

#[get("/encrypt/{password}")]
async fn encrypt_password(password: web::Path<String>) -> impl Responder {
    let hashed = hash(password.into_inner(), DEFAULT_COST).expect("Failed to hash password");
    HttpResponse::Ok().json(hashed)
}

#[post("/verify/{password}")]
async fn verify_password(password: web::Path<(String, String)>) -> impl Responder {
    let (password, hashed) = password.into_inner();
    match verify(password, &hashed) {
# FIXME: 处理边界情况
        Ok(valid) => HttpResponse::Ok().json(valid),
# 增强安全性
        Err(_) => HttpResponse::BadRequest().json("Invalid password or hash"),
    }
}

#[actix_web::main]
# 增强安全性
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
# 改进用户体验
            .service(encrypt_password)
            .service(verify_password)
# TODO: 优化性能
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
# 添加错误处理


/*
 * Notes:
 * - 使用bcrypt库进行密码加密和验证
 * - 可以通过GET和POST请求访问/encrypt和/verify接口
 * - 密码加密接口: /encrypt/{password}，返回加密后的密码
 * - 密码验证接口: /verify/{password}/{hashed}，返回验证结果
 * - 错误处理：使用期望值unwrap和匹配语句处理潜在错误
 * - 遵循RUST和ACTIX框架的最佳实践
 */
