use actix_web::{get, HttpResponse, Responder, web};
use magic::Magic;

/// 文件类型识别器服务
#[get("/file_type/{file_path}")] // 定义路由，接受文件路径参数
async fn file_type(file_path: web::Path<String>) -> impl Responder {
    let file_path = file_path.into_inner(); // 将文件路径从Path<String>类型转换为String

    // 创建Magic对象用于文件类型检测
    let mut magic = Magic::new(magic::MAGIC_MIME_TYPE);
    if let Err(e) = magic.load() {
        // 如果加载魔法库失败，返回错误响应
        return HttpResponse::InternalServerError().json(e.to_string());
    }

    // 尝试打开文件并读取数据用于文件类型检测
    let file_type = match std::fs::read(&file_path) {
        Ok(file_content) => {
            // 如果读取文件成功，使用Magic对象检测文件类型
            if let Some(mimetype) = magic.buffer(&file_content) {
                mimetype
            } else {
                // 如果文件类型检测失败，返回错误响应
                