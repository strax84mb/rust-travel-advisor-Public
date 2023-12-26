pub mod cities {
    use std::sync::Arc;

    use diesel::{
        insert_into,
        sql_function,
        prelude::*,
    };

    use crate::{
        model::City,
        schema::cities::dsl::*,
        util::{
            Error,
            ErrorCode::{
                DbRead,
                DbSave,
            },
        },
    };
    use super::super::{
        Database,
        entities::CityDB,
        db_context::db_macros::get_connection_v2,
    };

    pub trait CityRepository {
        fn get_all(&self) -> Result<Vec<City>, Error>;
        fn get_by_id(&self, id: i64) -> Result<Option<City>, Error>;
        fn new(&self, name: String) -> Result<City, Error>;
        fn get_by_name(&self, name: String) -> Result<Option<City>, Error>;
    }

    sql_function! { fn last_insert_id() -> BigInt; }

    struct CityRepositoryImpl {
        db: Arc<Database>,
    }

    pub fn new_city_repository(db: Arc<Database>) -> Arc<impl CityRepository> {
        Arc::new(CityRepositoryImpl {
            db: db,
        })
    }

    impl CityRepository for CityRepositoryImpl {

        fn get_all(&self) ->  Result<Vec<City> ,Error> {
            let conn = &mut get_connection_v2!(self.db);
            match cities.select(CityDB::as_select()).load(conn) {
                Ok(result) => Ok(result.iter().map(|c| c.to_city()).collect()),
                Err(err) => Err(Error::internal(DbRead, err.to_string())),
            }
        }

        fn get_by_id(&self, city_id: i64) -> Result<Option<City>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match cities
                .find(city_id)
                .select(CityDB::as_select())
                .first(conn)
                .optional() {
                    Ok(result) => match result {
                        Some(result) => Ok(Some(result.to_city())),
                        None => Ok(None),
                    },
                    Err(err) => Err(Error::internal(DbRead, err.to_string())),
                }
        }

        fn new(&self, city_name: String) -> Result<City, Error> {
            let conn = &mut get_connection_v2!(self.db);
            let result = insert_into(cities)
                .values(name.eq(city_name.clone()))
                .execute(conn);
            match result {
                Err(err) => return Err(Error::internal(DbSave, err.to_string())),
                Ok(size) if size > 0 => (),
                _ => return Err(Error::internal(DbSave, "nothing was inserted".to_string())),
            };
            match cities.select(last_insert_id()).load::<i64>(conn) {
                Err(err) => Err(Error::internal(DbRead, err.to_string())),
                Ok(result) if result.len() > 0 => Ok(City {
                    id: result[0],
                    name: city_name,
                    airports: vec![],
                    comments: vec![],
                }),
                _ => Ok(City {
                    id: -1,
                    name: city_name,
                    airports: vec![],
                    comments: vec![],
                }),
            }
        }

        fn get_by_name(&self, city_name: String) -> Result<Option<City>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match cities
                .filter(name.eq(city_name.clone()))
                .select(CityDB::as_select())
                .first(conn)
                .optional() {
                    Ok(result) => match result {
                        Some(result) => Ok(Some(result.to_city())),
                        None => Ok(None),
                    },
                    Err(err) => Err(Error::internal(DbRead, err.to_string())),
                }
        }

    }

}

#[cfg(test)]
mod db_test {
    #[test]
    fn test_list_columns() {
    }
}