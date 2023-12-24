pub mod airports {
    use std::sync::Arc;

    use diesel::prelude::*;

    use crate::{
        model::Airport,
        util::{
            Error,
            ErrorCode::{
                DbRead,
                DbSave,
                DbDelete,
                GetDbConnection,
            },
        },
        schema::airports as air_sch,
        storage::Database,
    };
    use super::super::{
        db_context::db_macros::get_connection_v2,
        entities::{
            AirportDB,
            InsertAirportDB,
        },
    };

    pub trait AirportRepository {
        fn get_all(&self) -> Result<Vec<Airport>, Error>;
        fn get_by_id(&self, id: i64) -> Result<Option<Airport>, Error>;
        fn new(&self, airport: &Airport) -> Result<Airport, Error>;
        fn update(&self, airport: Airport) -> Result<(), Error>;
        fn delete(&self, id: i64) -> Result<(), Error>;
        fn get_by_city_id(&self, city_id: i64) -> Result<Vec<Airport>, Error>;
    }

    struct AirportRepositoryImpl {
        db: Arc<Database>,
    }

    pub fn new_airport_repository(db: Arc<Database>) -> Arc<impl AirportRepository> {
        Arc::new(AirportRepositoryImpl {
            db: db,
        })
    }

    diesel::sql_function! { fn last_insert_id() -> BigInt; }

    impl AirportRepository for AirportRepositoryImpl {

        fn get_all(&self) -> Result<Vec<Airport>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match air_sch::dsl::airports.select(AirportDB::as_select()).load(conn) {
                Ok(result) => Ok(
                    result.iter().map(
                        |a| 
                        a.to_model()).collect()
                ),
                Err(err) => Err(Error::internal(DbRead, err.to_string())),
            }
        }

        fn get_by_id(&self, id: i64) -> Result<Option<Airport>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match air_sch::dsl::airports
                .find(id)
                .select(AirportDB::as_select())
                .first(conn)
                .optional() {
                    Ok(result) => match result {
                        Some(result) => Ok(Some(result.to_model())),
                        None => Ok(None),
                    },
                    Err(err) => Err(Error::internal(DbRead, err.to_string())),
                }
        }

        fn new(&self, airport: &Airport) -> Result<Airport, Error> {
            let conn = &mut get_connection_v2!(self.db);
            let trx_result = conn.transaction::<i64, diesel::result::Error, _>(|conn| {
                let entity = InsertAirportDB {
                    name: airport.name.clone(),
                    city_id: airport.city_id.clone(),
                };
                match diesel::insert_into(air_sch::dsl::airports)
                    .values(&entity)
                    .execute(conn) {
                        Err(err) => Err(err),
                        Ok(_) => {
                            match air_sch::dsl::airports.select(last_insert_id()).load::<i64>(conn) {
                                Err(err) => Err(err),
                                Ok(ids) if ids.len() > 0 => Ok(ids[0]),
                                _ => Ok(-1),
                            }
                        }
                    }
            });
            match trx_result {
                Ok(res) => Ok(Airport {
                    id: res,
                    city_id: airport.city_id.clone(),
                    name: airport.name.clone(),
                }),
                Err(err) => Err(Error::internal(DbSave, err.to_string())),
            }
        }

        fn update(&self, airport: Airport) -> Result<(), Error> {
            let conn = &mut get_connection_v2!(self.db);
            match diesel::update(air_sch::dsl::airports)
                .filter(air_sch::dsl::id.eq(airport.id.clone()))
                .set((
                    air_sch::dsl::city_id.eq(airport.city_id.clone()),
                    air_sch::dsl::name.eq(airport.name.clone()),
                )).execute(conn) {
                    Ok(rows) => match rows {
                        0 => Err(Error::not_found("airport not found".to_string())),
                        _ => Ok(()),
                    },
                    Err(err) => Err(Error::internal(DbSave, err.to_string())),
                }
        }

        fn delete(&self, id: i64) -> Result<(), Error> {
            let conn = &mut get_connection_v2!(self.db);
            match diesel::delete(air_sch::dsl::airports.filter(air_sch::dsl::id.eq(id))).execute(conn) {
                Err(err) => Err(Error::internal(DbDelete, err.to_string())),
                Ok(rows) => match rows {
                    0 => Err(Error::not_found("airport not found".to_string())),
                    _ => Ok(()),
                },
            }
        }

        fn get_by_city_id(&self, city_id: i64) -> Result<Vec<Airport>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match air_sch::dsl::airports
                .filter(air_sch::dsl::city_id.eq(city_id))
                .select(AirportDB::as_select())
                .load(conn) {
                    Ok(result) => Ok(result.iter().map(|a| a.to_model()).collect()),
                    Err(err) => Err(Error::internal(DbRead, err.to_string())),
                }
        }

    }

}