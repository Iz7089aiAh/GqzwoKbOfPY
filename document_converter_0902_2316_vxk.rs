use actix_web::{web, App, HttpServer, HttpResponse, Responder};

/// 定义一个结构体，用于处理文档转换请求
struct DocumentConverter;

/// 实现文档转换的函数，接受文档类型和内容
impl DocumentConverter {
    /// 将文档从一种格式转换为另一种格式
    ///
    /// 参数:
    /// * `from_format` - 文档的原始格式
    /// * `content` - 文档的内容
    ///
    /// 返回:
    /// * `Result<String, String>` - 转换后的文档内容或错误信息
    pub fn convert(&self, from_format: &str, content: &str) -> Result<String, String> {
        // 简单的示例，实际转换逻辑应根据需要实现
        match from_format {
            "txt" => Ok(format!("Converted from {} to html: <pre>{}</pre>", from_format, content)),
            _ => Err("Unsupported format".to_string()),
        }
    }
}

/// 实现Actix Web的业务逻辑
async fn convert_document(from_format: web::Path<String>, content: web::Json<String>) -> impl Responder {
    let converter = DocumentConverter;
    match converter.convert(&from_format, &content) {
        Ok(converted_content) => HttpResponse::Ok().json(converted_content),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

/// 定义主函数，启动HTTP服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/convert", web::post().to(convert_document))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
