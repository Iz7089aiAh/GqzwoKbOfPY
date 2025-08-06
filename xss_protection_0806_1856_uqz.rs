use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, HttpRequest};
use actix_web::middleware::Logger;
use regex::Regex;
use std::str::FromStr;
use percent_encoding::percent_decode_str;

// 一个简单的XSS攻击防护函数
// 它使用正则表达式来检测并删除潜在的XSS攻击代码
fn sanitize_input(input: &str) -> String {
    let mut sanitized_input = input.to_string();
    // 定义一个XSS攻击模式的正则表达式
    let xss_regex = Regex::new(r"<[^>]*>|["\'](.*?)["\']").unwrap();
# 优化算法效率
    
    // 移除HTML标签和转义的引号
# 改进用户体验
    sanitized_input = xss_regex.replace_all(&sanitized_input, "").to_string();
    
    // 解码百分号编码的字符来防止字符编码攻击
# 改进用户体验
    sanitized_input = percent_decode_str(&sanitized_input).decode_utf8_lossy().to_string();
    
    sanitized_input
}

// 根路由处理函数，返回一个简单的欢迎消息
#[get("/")]
async fn index() -> impl Responder {
    // 假设用户输入了一个潜在的XSS攻击代码
    let user_input = "<script>alert('XSS')</script>";
    
    // 清理用户输入
    let safe_input = sanitize_input(user_input);
    
    // 返回清理后的输入
    HttpResponse::Ok().body(safe_input)
# 扩展功能模块
}

// 主函数，设置和启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}