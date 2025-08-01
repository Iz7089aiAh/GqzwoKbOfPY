use actix_web::{web, HttpResponse, Responder, App, HttpServer, get, post, put, delete};

// 定义一个结构体来表示响应式布局的配置
#[derive(Debug)]
struct ResponsiveLayoutConfig {
    width: u32,
    height: u32,
    theme: String,
}

// 实现一个服务来处理响应式布局请求
struct ResponsiveService;

// 为ResponsiveService实现方法
impl ResponsiveService {
    // 获取响应式布局配置
    #[get("/config")]
    async fn get_config() -> impl Responder {
        let config = ResponsiveLayoutConfig {
            width: 1024,
            height: 768,
            theme: "light".to_string(),
        };

        HttpResponse::Ok().json(config)
    }

    // 更新响应式布局配置
    #[post("/config")]
    async fn update_config(config: web::Json<ResponsiveLayoutConfig>) -> impl Responder {
        // 这里可以添加错误处理和验证逻辑
        HttpResponse::Ok().json(config.into_inner())
    }
}

// 启动服务器的主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置服务器
    let server = HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/config")
                    .route(web::get().to(ResponsiveService::get_config))
                    .route(web::post().to(ResponsiveService::update_config)),
            ),
    })
    .bind("127.0.0.1:8080")?
    .run();

    println!("Server is running on http://127.0.0.1:8080/");
    server.await
}

// 单元测试可以在这里添加
// #[cfg(test)]
// mod tests {
//     use super::*;
//     // 测试用例
// }