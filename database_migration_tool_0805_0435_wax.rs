use actix_web::{web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

// 定义数据库配置
pub struct DatabaseConfig {
    pub url: String,
}

// 数据库连接池
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// 实现数据库迁移操作
fn run_migration() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取数据库连接字符串
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 创建数据库连接池
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    // 执行迁移操作
    embedded_migrations::run_with_output(&pool.get()?, embedded_migrations::run_offline)?;

    Ok(())
}

// 定义HTTP服务
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init();

    // 配置数据库
    let db_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            // 配置数据库连接池
            .app_data(web::Data::new(DbPool::builder()
                .build(ConnectionManager::<PgConnection>::new(&db_config.url))
                .expect("Failed to create pool.")))
            // 配置路由
            .service(web::resource("/migrate").route(web::post().to(migrate)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 定义迁移路由
async fn migrate(pool: web::Data<DbPool>) -> impl Responder {
    // 执行迁移操作
    match run_migration() {
        Ok(_) => Ok("Migration successful"),
        Err(e) => Err(actix_web::HttpResponse::InternalServerError().json(e.to_string())),
    }
}