use std::time::SystemTime;

use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: i64,
    pub user_id: i64,
    pub city_id: i64,
    pub content: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
