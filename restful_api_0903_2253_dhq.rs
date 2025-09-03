use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post, put, delete};

// 定义一个简单的数据结构来模拟数据库中的记录
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 定义一个模拟的数据存储
struct UserService;

impl UserService {
    // 创建新用户
    async fn create_user(user: User) -> Result<User, String> {
        Ok(user)
    }

    // 获取单个用户
    async fn get_user(id: u32) -> Result<User, String> {
        Err("User not found".to_string())
    }

    // 更新用户
    async fn update_user(id: u32, user: User) -> Result<User, String> {
        Ok(user)
    }

    // 删除用户
    async fn delete_user(id: u32) -> Result<(), String> {
        Ok(())
    }
}

// 实现HTTP GET请求，用于获取用户信息
#[get("/users/{id}")]
async fn get_user_handler(id: web::Path<u32>) -> impl Responder {
    match UserService::get_user(id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// 实现HTTP POST请求，用于创建新用户
#[post("/users")]
async fn create_user_handler(user: web::Json<User>) -> impl Responder {
    match UserService::create_user(user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// 实现HTTP PUT请求，用于更新用户信息
#[put("/users/{id}")]
async fn update_user_handler(id: web::Path<u32>, user: web::Json<User>) -> impl Responder {
    match UserService::update_user(id.into_inner(), user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// 实现HTTP DELETE请求，用于删除用户
#[delete("/users/{id}")]
async fn delete_user_handler(id: web::Path<u32>) -> impl Responder {
    match UserService::delete_user(id.into_inner()).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器，监听8080端口
    HttpServer::new(|| {
        App::new()
            // 注册路由和相应的处理器函数
            .route("/users/{id}", web::get().to(get_user_handler))
            .route("/users", web::post().to(create_user_handler))
            .route("/users/{id}", web::put().to(update_user_handler))
            .route("/users/{id}", web::delete().to(delete_user_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}