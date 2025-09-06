use actix_web::{HttpResponse, get, post, web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::json;
use r2d2_diesel::DieselConnection;

// 定义数据库连接配置
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct MigrationRequest {
    command: String,
}

#[get("/migrate")]
async fn migrate(db_pool: web::Data<DbPool>) -> impl Responder {
    let conn = db_pool.get().expect("Failed to get DB connection from pool.");

    // 执行迁移命令
    match migrate_database(&conn).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "Migration successful"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

#[post("/migrate")]
async fn migrate_post(req_body: web::Json<MigrationRequest>, db_pool: web::Data<DbPool>) -> impl Responder {
    let conn = db_pool.get().expect("Failed to get DB connection from pool.");

    // 根据请求参数执行不同的迁移命令
    match req_body.command.as_str() {
        "up" => migrate_up(&conn).await,
        "down" => migrate_down(&conn).await,
        _ => HttpResponse::BadRequest().json(json!({"error": "Invalid command"})),
    }
}

async fn migrate_database(conn: &PgConnection) -> Result<(), diesel::result::Error> {
    // 执行数据库迁移逻辑，例如使用diesel的.migrations()运行迁移脚本
    // 此处省略具体迁移代码，需要根据实际项目配置
    Ok(())
}

async fn migrate_up(conn: &PgConnection) -> impl Responder {
    // 执行向上迁移的逻辑
    // 此处省略具体代码
    HttpResponse::Ok().json(json!({"status": "Migration up successful"}))
}

async fn migrate_down(conn: &PgConnection) -> impl Responder {
    // 执行向下迁移的逻辑
    // 此处省略具体代码
    HttpResponse::Ok().json(json!({"status": "Migration down successful"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(migrate)
            .service(migrate_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
