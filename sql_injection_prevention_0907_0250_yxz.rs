// 使用 Rust 和 Actix 框架创建防止 SQL 注入的程序
// 代码结构清晰，易于理解，并包含适当的错误处理。
// 添加必要的注释和文档，遵循 Rust 最佳实践。
// 确保代码的可维护性和可扩展性。

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::pg::Pg;
use actix_web::http::StatusCode;
use diesel::result::Error as DieselError;

// 定义数据库模型
#[derive(Queryable)]
#[table_name = "users"]
struct User {
    id: i32,
    username: String,
    email: String,
}

#[get("/users/{username}")]
async fn get_user_by_username(conn: web::Data<PgConnection>, username: web::Path<String>) -> impl Responder {
    // 使用异步块来处理可能的错误
    async_block!(
        // 使用 Diesel 的查询构建器防止 SQL 注入
        let result = web::block(
            || conn.first(User.filter(User::username.eq(username.into())))).await;
        
        match result {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(e) => {
                eprintln!("Error fetching user: {:?}