use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use serde::Serialize;
use serde_json::json;

// 定义一个结构体来生成测试数据
#[derive(Serialize)]
struct TestData {
    id: u64,
    name: String,
    email: String,
    age: u8,
}

impl TestData {
    // 生成一个新的测试数据实例
    fn new(id: u64) -> Self {
        let mut rng = rand::thread_rng();

        TestData {
            id,
            name: format!("User{}", rng.gen::<u32>()),
            email: format!("user{}@example.com", rng.gen::<u32>()),
            age: rng.gen_range(18..100),
        }
    }
}

// 定义异步函数来生成测试数据
async fn generate_data() -> impl Responder {
    let test_data = TestData::new(1); // 示例中只生成一个测试数据，可根据需要生成多个
    HttpResponse::Ok().json(json!(test_data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate", web::get().to(generate_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 为结构体和函数添加文档注释，提高代码的可读性和可维护性
/// TestData 结构体用于表示测试数据
///
/// 包含 id, name, email 和 age 四个字段
///
/// # Examples
///
/// ```rust
/// let test_data = TestData::new(1);
/// println!("Generated Test Data: {:?}", test_data);
/// ```

/// generate_data 函数用于生成测试数据
///
/// 该函数返回一个响应，其中包含生成的测试数据
///
/// # Examples
///
/// ```rust
/// let response = generate_data().await;
/// assert!(response.is_ok());
/// ```