use actix::prelude::*;
use actix_web::{
    HttpResponse,
    web,
    App,
    HttpServer,
};

/// 消息通知服务
struct NotificationService;

/// 消息请求结构体
#[derive(Deserialize)]
struct NotificationRequest {
    message: String,
}

impl NotificationRequest {
    /// 创建新的 NotificationRequest 实例
    pub fn new(message: &str) -> Self {
        NotificationRequest {
            message: message.to_owned(),
        }
    }
}

/// 消息通知服务实现
impl MessageNotification for NotificationService {
    /// 发送消息
    fn notify(&self, request: NotificationRequest) -> impl MessageResponse {
        // 这里可以添加实际的消息发送逻辑，例如发送邮件、推送通知等
        println!("Sending notification: {}", request.message);

        // 模拟发送成功
        Ok(NotificationResponse::Success)
    }
}

/// 消息响应枚举
enum NotificationResponse {
    Success,
    Error(String),
}

/// 消息通知服务 trait
trait MessageNotification: MessageResponse + Send + Sync {
    /// 发送消息通知
    fn notify(&self, request: NotificationRequest) -> NotificationResponse;
}

/// 消息响应 trait
trait MessageResponse: Message + Send {
    fn response(&self) -> HttpResponse;
}

impl MessageResponse for NotificationResponse {
    fn response(&self) -> HttpResponse {
        match self {
            NotificationResponse::Success => HttpResponse::Ok().into(),
            NotificationResponse::Error(err) => HttpResponse::InternalServerError().json(err),
        }
    }
}

/// 消息通知服务 handler
async fn notification_service_handler(
    req: NotificationRequest,
) -> impl MessageResponse {
    let service = NotificationService;
    service.notify(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/notify", web::post().to(notification_service_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
