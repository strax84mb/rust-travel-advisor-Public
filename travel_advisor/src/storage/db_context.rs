use std::sync::Arc;

use diesel::{
    mysql::MysqlConnection,
    r2d2::ConnectionManager,
};
use r2d2::Pool;

pub struct Database {
    pub conns: Arc<Pool<ConnectionManager<MysqlConnection>>>
}

impl Database {
    pub async fn new(url: String) -> Result<Database, String> {
        let manager = ConnectionManager::<MysqlConnection>::new(url);
        let pool = Pool::new(manager).expect("failed to connect to DB");
        Ok(Database {
            conns: Arc::new(pool),
         })
    }
}

#[macro_use]
pub mod db_macros {

    macro_rules! get_connection_v2 {
        ($payload:expr) => {
            match $payload.conns.to_owned().get() {
                Ok(connection) => connection,
                Err(err) => return Err(Error::internal(crate::util::ErrorCode::GetDbConnection, err.to_string())),
            }
        };
    }

    pub(crate) use get_connection_v2;
}