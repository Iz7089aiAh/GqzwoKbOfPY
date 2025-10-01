use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

// 排行榜条目结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct LeaderboardEntry {
    id: u32,
    name: String,
    score: u32,
}

// 排行榜服务
# 添加错误处理
struct LeaderboardService {
    entries: Mutex<HashMap<u32, LeaderboardEntry>>,
}

impl LeaderboardService {
    // 构造函数
    fn new() -> LeaderboardService {
        LeaderboardService {
            entries: Mutex::new(HashMap::new()),
        }
    }

    // 添加或更新排行榜条目
    fn add_or_update_entry(&self, entry: LeaderboardEntry) {
        let mut entries = self.entries.lock().unwrap();
        entries.insert(entry.id, entry);
    }

    // 获取排行榜
    fn get_leaderboard(&self) -> Vec<LeaderboardEntry> {
        let entries = self.entries.lock().unwrap();
        entries.values().cloned().collect()
    }
}

// HTTP 处理器
async fn add_entry(service: web::Data<Mutex<LeaderboardService>>, entry: web::Json<LeaderboardEntry>) -> impl Responder {
# FIXME: 处理边界情况
    let mut service = service.lock().unwrap();
    service.add_or_update_entry(entry.into_inner());
    HttpResponse::Ok()
# 扩展功能模块
}
# 改进用户体验

async fn get_leaderboard(service: web::Data<Mutex<LeaderboardService>>) -> impl Responder {
# 增强安全性
    let service = service.lock().unwrap();
    let entries = service.get_leaderboard();
    HttpResponse::Ok().json(entries)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化排行榜服务
    let leaderboard_service = web::Data::new(Mutex::new(LeaderboardService::new()));

    // 创建HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(leaderboard_service.clone())
# 改进用户体验
            .service(add_entry)
            .service(get_leaderboard)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
