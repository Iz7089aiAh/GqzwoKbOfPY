use actix_web::{get, HttpResponse, Responder};
# 添加错误处理
use serde::{Serialize, Deserialize};
use std::fmt;

// 定义一个结构体来表示物体的位置和大小
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Object {
    x: f64,
    y: f64,
# FIXME: 处理边界情况
    width: f64,
    height: f64,
}

// 定义一个结构体来表示碰撞检测的结果
#[derive(Serialize, Deserialize, Debug)]
struct CollisionResult {
    collided: bool,
    message: String,
}

// 实现一个方法来检测两个物体是否发生碰撞
impl Object {
    fn detect_collision(&self, other: &Object) -> CollisionResult {
# 增强安全性
        if self.x < other.x + other.width &&
           self.x + self.width > other.x &&
           self.y < other.y + other.height &&
           self.y + self.height > other.y {
            CollisionResult {
                collided: true,
# 改进用户体验
                message: "Collision detected".to_string(),
            }
        } else {
            CollisionResult {
                collided: false,
                message: "No collision".to_string(),
# NOTE: 重要实现细节
            }
        }
    }
}

// 定义一个服务来处理碰撞检测的请求
#[derive(Debug, Clone)]
struct CollisionService;

impl CollisionService {
    #[get("/detect")]
    async fn detect(&self) -> impl Responder {
# 扩展功能模块
        let object1 = Object { x: 10.0, y: 20.0, width: 50.0, height: 30.0 };
        let object2 = Object { x: 60.0, y: 70.0, width: 40.0, height: 25.0 };

        let result = object1.detect_collision(&object2);

        HttpResponse::Ok().json(result)
    }
}

fn main() {
    // 设置Actix Web服务器
# 添加错误处理
    let service = actix_web::web::service(
        CollisionService::new(),
    );
    
    // 启动服务器监听8080端口
    actix_web::web::new_service()
        .wrap(actix_web::middleware::Logger::default())
        .start("127.0.0.1:8080".parse().unwrap())
        .unwrap()
        .run(service)
        .unwrap();
}
# 改进用户体验

// 格式化输出以便于阅读
# 增强安全性
impl fmt::Display for Object {
# NOTE: 重要实现细节
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object {{ x: {}, y: {}, width: {}, height: {} }}", self.x, self.y, self.width, self.height)
    }
}

// 格式化输出以便于阅读
impl fmt::Display for CollisionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CollisionResult {{ collided: {}, message: '{}' }}", self.collided, self.message)
    }
}
# TODO: 优化性能