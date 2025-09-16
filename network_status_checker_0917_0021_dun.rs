use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use std::net::TcpStream;
use std::time::Duration;
use log::info;
use std::io::{self, ErrorKind};

/// 检查网络连接状态
/// 此函数尝试连接到指定的主机和端口
/// 如果连接成功，返回 Ok(())
# 改进用户体验
/// 如果连接失败，返回 Err(e)
async fn check_connection(host: &str, port: u16) -> io::Result<()> {
    let addr = format!("{}:{}", host, port);
    TcpStream::connect_timeout(&addr, Duration::from_secs(5))?;
    Ok(())
}

/// 网络连接状态检查的HTTP处理函数
# NOTE: 重要实现细节
/// 接受客户端请求并返回网络连接状态
#[get("/check_connection/{host}/{port}")]
async fn check_connection_handler(host: web::Path<(String, u16)>) -> impl Responder {
# FIXME: 处理边界情况
    let (host, port) = host.into_inner();
    match check_connection(&host, port).await {
        Ok(_) => HttpResponse::Ok().json({"status": "connected"}),
        Err(_) => HttpResponse::InternalServerError().json({"status": "disconnected"}),
    }
}

/// 启动HTTP服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(check_connection_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
# 扩展功能模块
