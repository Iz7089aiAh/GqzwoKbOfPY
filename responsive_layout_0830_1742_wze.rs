use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, RespondeToMessages, MessageBody, ErrorBadRequest};

// 定义一个消息类型，用于响应式布局的请求
#[derive(MessageBody)]
struct ResponsiveLayoutMessage {
    // 可以添加更多字段来描述布局的需求
    width: u32,
    height: u32,
}

// 定义响应结构体
#[derive(Responder)]
struct ResponsiveLayoutResponse {
    // 可以根据实际需求定义更多的字段
    layout: String,
}

// 控制器结构体，包含处理逻辑
struct LayoutController;

impl LayoutController {
    // 根据屏幕大小返回响应式布局
    #[get("/layouts/{width}/{height}")]
    async fn layout(&self, width: web::Path<u32>, height: web::Path<u32>) -> impl Responder {
        let (width, height) = (width.into_inner(), height.into_inner());

        // 错误处理：如果输入的尺寸不合理，返回错误信息
        if width == 0 || height == 0 {
            return ErrorBadRequest("Width and height must be greater than zero");
        }

        // 根据屏幕大小生成响应式布局
        let layout = match (width, height) {
            // 可以添加更多的匹配条件来处理不同的屏幕尺寸
            (w, h) if w > 800 && h > 600 => "Desktop Layout".to_string(),
            (w, h) if w > 480 => "Tablet Layout".to_string(),
            _ => "Mobile Layout".to_string(),
        };

        HttpResponse::Ok().json(ResponsiveLayoutResponse { layout })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            // 注册路由
            .service(LayoutController::layout)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
