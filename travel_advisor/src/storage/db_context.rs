use std::sync::Arc;
use sqlx::mysql::MySqlPool;

pub struct Database {
    pub connections: Arc<MySqlPool>,
}

impl Database {
    pub async fn new(url: String) -> Result<Database, String> {
        let pool = match MySqlPool::connect(url.as_str()).await {
            Ok(result) => Arc::new(result),
            Err(err) => return Err(err.to_string()),
        };

        Ok(Database {
            connections: pool,
        })
    }
}
