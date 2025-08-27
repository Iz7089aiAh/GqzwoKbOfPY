// database_pool_manager.rs
// 这个模块将负责管理数据库连接池。

use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use r2d2_diesel::ConnectionManager;

// 定义数据库连接池管理器
pub struct DbExecutor(
    r2d2::Pool<ConnectionManager<PgConnection>>
);

impl DbExecutor {
    // 创建一个新的数据库连接池
    pub fn new(database_url: &str) -> DbExecutor {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create pool.");
        DbExecutor(pool)
    }
}

// 实现Arbitrary，用于在Actix系统中传递数据库连接
impl Arbiter for DbExecutor {
    fn acquire(&self) -> r2d2::PooledConnection<ConnectionManager<PgConnection>> {
        self.0.get().expect("Failed to acquire connection")
    }
}

// 实现Handler，用于处理数据库查询请求
impl Handler<DbQuery> for DbExecutor {
    type Result = ResponseFuture<QueryResult>;

    fn handle(&mut self, msg: DbQuery, ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async move {
            let conn = self.acquire();
            let result = msg.run(&conn).await;
            ResponseFuture::new(future::ready(result))
        })
    }
}

// 定义数据库查询请求消息
pub struct DbQuery(
    Box<dyn FnOnce(&PgConnection) -> QueryResult + Send + 'static>,
);

impl Message for DbQuery {
    type Result = QueryResult;
}

// 定义查询结果
pub struct QueryResult(
    diesel::result::QueryResult
);

// 实现查询结果的From，用于从diesel::result::QueryResult转换
impl From<diesel::result::QueryResult> for QueryResult {
    fn from(item: diesel::result::QueryResult) -> Self {
        QueryResult(item)
    }
}

// 一个简单的示例，展示如何创建和使用数据库连接池
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 创建数据库连接池
    let db_executor = DbExecutor::new("postgres://user:password@localhost/database");

    // 发送数据库查询请求
    let query = DbQuery(Box::new(|conn| {
        let result = users::table
            .load::<User>(conn)
            .expect("Error loading users");
        Ok(result)
    }));

    // 处理查询结果
    let result = db_executor.send(query).await;
    match result {
        Ok(users) => println!("Loaded users: {:?}