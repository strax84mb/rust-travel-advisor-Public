use std::time::{SystemTime, Duration};

use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow,
    mysql::MySqlRow,
    Row,
    types::chrono::{DateTime, Utc}
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: i64,
    pub user_id: i64,
    pub city_id: i64,
    pub content: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime
}

impl <'c> FromRow<'c, MySqlRow> for Comment {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id: i64 = match row.try_get("id") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let user_id: i64 = match row.try_get("user_id") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let city_id: i64 = match row.try_get("city_id") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let content: String = match row.try_get("text") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let created_at: DateTime<Utc> = match row.try_get("updated_at") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let updated_at: DateTime<Utc> = match row.try_get("created_at") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        Ok(Comment {
            id: id,
            user_id: user_id,
            city_id: city_id,
            content: content,
            created_at: SystemTime::UNIX_EPOCH + Duration::from_nanos(created_at.timestamp() as u64),
            updated_at: SystemTime::UNIX_EPOCH + Duration::from_nanos(updated_at.timestamp() as u64)
        })
    }
}
