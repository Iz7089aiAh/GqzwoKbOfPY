// sql_injection_prevention.rs

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

// 定义数据库模型
#[derive(Queryable)]
struct User {
    id: i32,
    username: String,
    email: String,
}

// 定义数据库连接
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// 获取用户信息的函数，防止SQL注入
async fn get_user_info(user_id: web::Path<i32>) -> impl Responder {
    let connection = establish_connection();
    let user = web::block(move || {
        use schema::users::dsl::*;
        users.find(user_id.into_inner()).load::<User>(&connection)
    }).await;
    
    match user {
        Ok(users) => {
            if let Some(user) = users.first() {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::NotFound().finish()
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user/{user_id}", web::get().to(get_user_info))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

// schema模块
mod schema {
    pub mod users;
}

// users表定义
mod users {
    use super::schema::users;
    use diesel::Queryable;
    pub fn find(user_id: i32) -> users::BoxedQuery<'static, diesel::sql_query::SqlQuery> {
        users::table.filter(users::id.eq(user_id)).into_boxed()
    }
}

// schema模块
mod schema {
    use diesel::pg::Pg;
    pub mod users;
}

// users表定义
mod users {
    use super::Pg;
    use diesel::table;
    table! {
        users (id) {
            id -> Int4,
            username -> Varchar,
            email -> Varchar,
        }
    }
}
