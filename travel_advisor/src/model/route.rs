
use crate::util::{
    Error,
    ErrorCode::TextRowParse,
};
use super::common::FromStringRecord;

#[derive(Debug)]
pub struct Route {
    pub id: i64,
    pub start: i64,
    pub finish: i64,
    pub price: i64,
}

impl FromStringRecord for Route {
    type Output = Route;

    fn from_string_record(record: csv::StringRecord) -> Result<Self::Output, Error> {
        let start = match record[0].parse::<i64>() {
            Ok(v) => v,
            Err(err) => return Err(Error::internal(
                TextRowParse,
                format!("bad start city ID: {}", err.to_string())
            )),
        };

        let finish = match record[1].parse::<i64>() {
            Ok(v) => v,
            Err(err) => return Err(Error::internal(
                TextRowParse,
                format!("bad finish city ID: {}", err.to_string())
            )),
        };

        let price = match record[2].parse::<i64>() {
            Ok(v) => v,
            Err(err) => return Err(Error::internal(
                TextRowParse,
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