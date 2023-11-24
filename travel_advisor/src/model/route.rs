use serde::{
    Serialize,
    Deserialize,
};
use sqlx::{
    FromRow,
    mysql::MySqlRow,
    Row,
};

use super::common::FromStringRecord;

use crate::util::app_errors::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub id: i64,
    pub start: i64,
    pub finish: i64,
    pub price: i64,
}

impl<'c> FromRow<'c, MySqlRow> for Route {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id: i64 = match row.try_get("id") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let start: i64 = match row.try_get("start") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let finish: i64 = match row.try_get("finish") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let price: i64 = match row.try_get("price") {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        Ok(Route {
            id: id,
            start: start,
            finish: finish,
            price: price,
        })
    }
}

impl FromStringRecord for Route {
    type Output = Route;

    fn from_string_record(record: csv::StringRecord) -> Result<Self::Output, Error> {
        let start = match record[0].parse::<i64>() {
            Ok(v) => v,
            Err(err) => return Err(Error::underlying(
                format!("bad start city ID: {}", err.to_string())
            )),
        };

        let finish = match record[1].parse::<i64>() {
            Ok(v) => v,
            Err(err) => return Err(Error::underlying(
                format!("bad finish city ID: {}", err.to_string())
            )),
        };

        let price = match record[2].parse::<i64>() {
            Ok(v) => v,
            Err(err) => return Err(Error::underlying(
                format!("bad price: {}", err.to_string())
            )),
        };
        
        Ok(Route {
            id: 0,
            start: start,
            finish: finish,
            price: price,
        })
    }
}