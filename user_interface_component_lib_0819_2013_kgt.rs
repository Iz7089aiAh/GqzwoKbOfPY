use actix_web::{get, HttpResponse, Responder, web};
use serde::Serialize;
use serde_json::json;

// 定义用户界面组件的数据结构
#[derive(Serialize, Debug, Clone)]
pub struct UiComponent {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
}

// 定义一个错误类型来处理组件库中的错误
#[derive(Debug)]
pub enum UiComponentError {
    ComponentNotFound,
    InvalidComponentData,
}

// 实现错误处理，使其可以被`actix_web`框架识别
impl std::fmt::Display for UiComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            UiComponentError::ComponentNotFound => write!(f, "Component not found"),
            UiComponentError::InvalidComponentData => write!(f, "Invalid component data"),
        }
    }
}

impl actix_web::ResponseError for UiComponentError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UiComponentError::ComponentNotFound => HttpResponse::NotFound().json(json!({
                "error": self.to_string(),
            })),
            UiComponentError::InvalidComponentData => HttpResponse::BadRequest().json(json!({
                "error": self.to_string(),
            })),
        }
    }
}

// 定义一个服务来处理UI组件的请求
pub struct UiComponentService;

// 使用`get`宏标记这个函数，使其成为HTTP GET路由
#[get("/components/{name}")]
async fn get_component(ui_component: web::Path<String>) -> Result<impl Responder, UiComponentError> {
    // 模拟一个组件查找过程
    let component_name = ui_component.into_inner();
    match find_component(&component_name).await {
        Some(component) => Ok(HttpResponse::Ok().json(component)),
        None => Err(UiComponentError::ComponentNotFound),
    }
}

// 模拟查找组件的异步函数
async fn find_component(name: &str) -> Option<UiComponent> {
    // 这里应该是数据库查找或其他存储查找
    // 为了演示，我们使用一个简单的映射
    let components = vec![
        UiComponent {
            name: "Button".to_string(),
            description: "A simple button".to_string(),
            version: "1.0.0".to_string(),
            author: "John Doe".to_string(),
        },
        // 可以添加更多的组件
    ];
    components.into_iter().find(|c| c.name == name)
}

// 定义Actix应用并注册路由
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting UI Component Library...");
    // 创建一个服务配置，并添加我们的组件路由
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(get_component)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
