use serde::{Serialize, Deserialize};
use sqlx::{FromRow, mysql::MySqlRow, Row};

#[derive(Serialize, Deserialize, Clone)]
pub struct Airport {
    pub id: i64,
    pub city_id: i64,
    pub name: String
}

impl<'c> FromRow<'c, MySqlRow> for Airport {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id: i64 = match row.try_get("id") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let city_id: i64 = match row.try_get("city_id") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let name: String = match row.try_get("name") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        Ok(Airport {
            id: id,
            city_id: city_id,
            name: name
        })
    }
}
