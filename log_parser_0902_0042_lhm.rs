use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// LogParser 结构体用于处理日志文件解析
pub struct LogParser;

impl LogParser {
    /// 解析日志文件
    /// 
    /// # 参数
    /// * `path` - 日志文件的路径
    /// 
    /// # 返回值
    /// 解析后的日志行数
    pub fn parse_log(path: String) -> Result<usize, io::Error> {
        let path = Path::new(&path);
        let file = fs::File::open(&path)?;
        let reader = BufReader::new(file);

        let mut count = 0;
        for line in reader.lines() {
            let line = line?;
            // 这里可以根据需要添加日志解析逻辑
            // 例如，可以提取日志中的错误信息等
            println!("解析日志行: {}", line);
            count += 1;
        }

        Ok(count)
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let server_address = "127.0.0.1:8080";

    HttpServer::new(|| {
        App::new()
            .route("/parse", web::post().to(parse_log))
    })
    .bind(server_address)?
    .run()
    .await
}

/// 解析日志文件的HTTP处理函数
/// 
/// # 参数
/// * `content` - 日志文件的路径
/// 
/// # 返回值
/// HTTP响应对象
async fn parse_log(content: web::Json<String>) -> impl Responder {
    match LogParser::parse_log(content.into_inner()) {
        Ok(count) => HttpResponse::Ok().json({"count": count}),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
