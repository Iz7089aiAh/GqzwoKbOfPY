use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

// 定义数据库配置
pub fn init_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// 定义数据库模型
#[derive(Queryable)]
struct User {
    id: i32,
    username: String,
}

// 定义一个简单的用户查询
#[get("/users/{username}")]
async fn user_info(pool: web::Data<Pool<ConnectionManager<PgConnection>>>, username: web::Path<String>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection");

    let user = web::block(move || {
        use schema::users::dsl::*;

        users.filter(username.eq(username)).first(&conn)
    }).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 定义路由
fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(init_db_pool())
            .service(user_info)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}