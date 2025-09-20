use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

// 用户界面组件定义
#[derive(Debug, Deserialize)]
struct UiComponent {
    name: String,
    props: HashMap<String, String>,
}

// 组件库服务
struct UiComponentService;

impl UiComponentService {
    // 获取组件
    #[get("/components/{component_name}")]
    async fn get_component(
        &self,
        web::Path(component_name): web::Path<String>,
    ) -> impl Responder {
        match self.find_component(&component_name) {
            Some(component) => HttpResponse::Ok().json(json!(component)),
            None => HttpResponse::NotFound().json(json!({
                "error": "Component not found"
            })),
        }
    }

    // 模拟查找组件的方法
    fn find_component(&self, component_name: &str) -> Option<UiComponent> {
        // 这里可以接入真实的数据库或者资源库来查找组件
        // 仅为示例，返回硬编码的组件
        let components: HashMap<&str, UiComponent> = HashMap::from([
            ("button", UiComponent {
                name: "button".to_string(),
                props: HashMap::from([("color", "blue")])
            }),
            ("input", UiComponent {
                name: "input".to_string(),
                props: HashMap::from([("type", "text")])
            }),
        ]);
        components.get(component_name).cloned()
    }
}

// 程序的入口点
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(UiComponentService::get_component)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
