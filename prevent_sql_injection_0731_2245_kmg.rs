// Rust 程序防止SQL注入示例
// 使用 Actix 框架和 Diesel ORM库

// 引入所需库
use actix_web::{get, HttpResponse, Responder, web};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::query_builder::AsQuery;

// 定义数据库模型
#[derive(Queryable)]
struct User {
    id: i32,
    username: String,
}

// 定义数据库连接
struct Db(PgConnection);

// 实现 Send 和 Sync 以便可以在 Actix 线程池中使用数据库连接
impl<'a> actix_web::FromRequest<'a> for Db {
    type Error = actix_web::Error;
    type Future = actix_web::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &'a actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        // 从应用状态中获取数据库连接
        let pool = req.app_data::<Self::Config>().unwrap();
        let conn = pool.get().unwrap();
        Box::pin(async move { Ok(Db(conn)) })
    }
}

#[get("/users/{username}")]
async fn get_user(username: web::Path<String>, db: Db) -> impl Responder {
    // 使用 Diesel 构建 SQL 查询，防止 SQL 注入
    let user = diesel::sql_query("SELECT * FROM users WHERE username = $1")
        .bind::<String, _>(username.into_inner())
        .load::<User>(&db.0)
        .expect("加载用户失败");

    // 检查用户名是否存在
    match user.first() {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

fn main() -> std::io::Result<()> {
    // 设置数据库连接池
    let database_url = "postgres://username:password@localhost/dbname";
    let connection_pool = diesel::r2d2::Pool::builder()
        .build(diesel::r2d2::ConnectionManager::<User>::new(database_url))
        .expect("数据库连接池构建失败");

    // 启动 Actix 服务器
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::AppData::new(connection_pool.clone()))
            .route("/users/{username}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}