use actix_web::{get, post, put, web, HttpResponse, Responder, App, HttpServer, Responder as _};
use serde::Deserialize;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;

// 定义全局主题设置
lazy_static! {
    static ref THEME_SETTINGS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// 主题设置请求体结构
#[derive(Deserialize, Debug)]
struct ThemeSetting {
    theme: String,
}

// 定义一个简单的应用状态
struct AppState {}

// 获取当前主题的API
#[get("/theme")]
async fn get_theme() -> impl Responder {
    let themes = THEME_SETTINGS.lock().unwrap();
    HttpResponse::Ok().json(themes.clone())
}

// 设置主题的API
#[post("/theme")]
async fn set_theme(payload: web::Json<ThemeSetting>) -> impl Responder {
    let mut themes = THEME_SETTINGS.lock().unwrap();
    themes.insert("current_theme".to_string(), payload.theme.clone());
    HttpResponse::Ok().json({"theme": payload.theme})
}

// 更新主题的API
#[put("/theme")]
async fn update_theme(payload: web::Json<ThemeSetting>) -> impl Responder {
    set_theme(payload).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化全局主题设置
    THEME_SETTINGS.lock().unwrap().insert("current_theme".to_string(), "light".to_string());

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {}))
            .service(get_theme)
            .service(set_theme)
            .service(update_theme)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
