// database_migration_tool.rs
//
// 这是一个使用RUST和Actix框架的数据库迁移工具。

use actix_web::{web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::str::FromStr;
use dotenv::dotenv;
use diesel::pg::PgConnection;
use diesel_migrations::RunMigrationsError;
use diesel_migrations::{find_migrations, Migration, MigrationDirection, Runner, MigrationConnection, MigrationBox};

// 设置数据库连接池
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("DATABASE_URL", "postgres://username:password@localhost/database");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(
        database_url.as_str()
    );
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/migrate", web::post().to(migrate_handler))
    })
    .bind(format!("0.0.0.0:{}", port).as_str())?
    .run()
    .await
}

// 处理迁移请求
async fn migrate_handler(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let runner = Runner::new(&conn);
    let migrations = find_migrations!();
    match runner.run_migrations(&migrations, MigrationDirection::Up) {
        Ok(_) => Ok("Migrations applied successfully."),
        Err(RunMigrationsError::ApplyingMigrationError(_, err)) => Err(format!("Failed to apply migration: {}", err)),
        Err(RunMigrationsError::IoError(err)) => Err(format!("I/O error: {}", err)),
        Err(RunMigrationsError::VersionAlreadyExists(err)) => Err(format!("Migration version already exists: {}", err)),
    }
}
