use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Error};
use diesel::prelude::*;
use diesel::pg::PgConnection;

// 定义数据库模型
#[derive(Queryable)]
struct User {
    id: i32,
    username: String,
}

// 定义数据库模式
#[derive(Insertable)]
#[table_name = "users"]]
struct NewUser<'a> {
    username: &'a str,
}

// 定义数据库连接
struct Database {
    connection: PgConnection,
}

// 将数据库连接添加到Actix的AppState
impl actix_web::dev::Transform for Database {
    fn transform(&self, service: App) -> App {
        service.app_data(self.connection.clone())
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the SQL Injection prevention demo!")
}

#[get("/users/{username}")]
async fn user(db: web::Data<PgConnection>, username: web::Path<String>) -> Result<HttpResponse, Error> {
    // 使用参数化的查询防止SQL注入
    let result = db
        .table("users")
        .filter(diesel::dsl::sql("username = ?").bind::<&str>(username.as_str()))
        .load::<User>(&**db)
        .await;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error fetching user".to_string())),
    }
}

#[get("/add_user")]
async fn add_user(db: web::Data<PgConnection>, user: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    // 使用Diesel的Insertable trait防止SQL注入
    let conn = db.get_ref();
    let result = diesel::insert_into("users")
        .values(user)
        .execute(conn);

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("User added successfully".to_string())),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error adding user".to_string())),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置数据库连接
    let database = Database {
        connection: PgConnection::establish("postgres://username:password@localhost/database_name")
            .expect("Error connecting to database"),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.connection.clone()))
            .service(index)
            .service(user)
            .service(add_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}