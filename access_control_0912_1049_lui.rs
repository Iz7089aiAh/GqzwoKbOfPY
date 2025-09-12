use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware, get, post, Put, Delete};
    use serde::Deserialize;
    use serde_json::json;

    // 定义用户结构体，包含权限字段
    #[derive(Deserialize)]
    struct User {
        username: String,
        roles: Vec<String>,
    }

    // 定义访问控制中间件
    struct AccessControl;

    // 实现中间件的配置函数
    impl<S> middleware::Middleware<S> for AccessControl {
        fn start(&self, req: &HttpRequest) -> Result<middleware::Started, middleware::Error> {
            // 检查请求头中是否包含有效的认证信息
            if let Some(auth_header) = req.headers().get("Authorization") {
                // 假设认证信息格式为 "Bearer <token>"
                let token = auth_header.to_str().unwrap().trim_start_matches("Bearer ");
                // 调用函数验证token是否有效
                if self.validate_token(token) {
                    Ok(middleware::Started::Done)
                } else {
                    Err(middleware::Error::Internal)
                }
            } else {
                // 如果没有认证信息，返回401 Unauthorized
                Err(middleware::Error::from(HttpResponse::Unauthorized().finish()))
            }
        }
    }

    impl AccessControl {
        // 验证token是否有效
        fn validate_token(&self, token: &str) -> bool {
            // 这里应该是验证token的逻辑，假设token为有效
            true
        }

        // 检查用户是否有权限访问资源
        fn has_permission(&self, user_roles: &[String], required_role: &str) -> bool {
            // 检查用户角色是否包含所需角色
            user_roles.contains(&required_role.to_string())
        }
    }

    // 实现路由处理器
    async fn index() -> impl Responder {
        HttpResponse::Ok().body("This is a public endpoint")
    }

    // 实现需要权限的路由处理器
    #[get("/private")]
    async fn private_area(user: web::Json<User>) -> impl Responder {
        if AccessControl::has_permission(&user.roles, "admin") {
            HttpResponse::Ok().body("Access granted to private area")
        } else {
            HttpResponse::Forbidden().body("Access denied")
        }
    }

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                // 添加访问控制中间件
                .wrap(middleware::DefaultHeaders::new().add(