use actix_web::{web, App, HttpServer, Responder, error::ErrorInternalServerError, get};
use serde::Deserialize;
use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::distributions::Standard;

// 定义一个结构体来存储测试数据的配置
#[derive(Debug, Deserialize)]
pub struct TestDataConfig {
    pub count: u32,
    pub length: u32,
}

// 实现一个函数来生成测试数据
async fn generate_test_data(config: web::Json<TestDataConfig>) -> impl Responder {
    let TestDataConfig { count, length } = config.into_inner();

    let mut rng = rand::thread_rng();
    let test_data: Vec<String> = (0..count).map(|_| {
        let sample: String = rng.sample_iter(&Alphanumeric).take(length as usize).map(char::from).collect();
        sample
    }).collect();

    match test_data.len() {
        0 => Err(ErrorInternalServerError("Failed to generate test data".to_string())),
        _ => Ok(test_data),
    }
}

// 定义主函数，设置路由并启动HTTP服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/test-data", get().to(generate_test_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
