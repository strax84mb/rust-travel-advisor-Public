pub mod routes {
    use std::sync::Arc;

    use diesel::{
        sql_function,
        prelude::*,
    };

    use crate::{
        model::Route,
        schema::routes::dsl as route_dsl,
        storage::Database,
        util::{
            Error,
            ErrorCode::{
                DbRead,
                DbSave,
            },
        },
    };
    use super::super::{
        db_context::db_macros::get_connection_v2,
        entities::{
            RouteDB,
            InsertRouteDB,
        },
    };

    sql_function! { fn last_insert_id() -> BigInt; }

    pub trait RouteRepository {
        fn get_all(&self, offset: i64, limit: i64) -> Result<Vec<Route>, Error>;
        fn find_by_id(&self, id: i64) -> Result<Option<Route>, Error>;
        fn new(&self, route: Route) -> Result<Route, Error>;
        fn update(&self, route: Route) -> Result<(), Error>;
        fn delete(&self, id: i64) -> Result<(), Error>;
        fn find_by_start(&self, start: i64, exclude_finishes: Option<Vec<i64>>) -> Result<Vec<Route>, Error>;
    }

    struct RouteRepositoryImpl {
        db: Arc<Database>,
    }

    pub fn new_route_repository(db: Arc<Database>) -> Arc<impl RouteRepository> {
        Arc::new(RouteRepositoryImpl {
            db: db,
        })
    }

    impl RouteRepository for RouteRepositoryImpl {
        fn get_all(&self, offset: i64, limit: i64) -> Result<Vec<Route>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match route_dsl::routes
                .select(RouteDB::as_select())
                .offset(offset)
                .limit(limit)
                .load(conn) {
                    Ok(result) => Ok(result.iter().map(|r| r.to_model()).collect()),
                    Err(err) => Err(Error::internal(DbRead, err.to_string())),
                }
        }

        fn find_by_id(&self, id: i64) -> Result<Option<Route>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            match route_dsl::routes
                .find(id)
                .select(RouteDB::as_select())
                .first(conn)
                .optional() {
                    Ok(result_option) => match result_option {
                        Some(result) => Ok(Some(result.to_model())),
                        None => Ok(None),
                    },
                    Err(err) => Err(Error::internal(DbRead, err.to_string())),
                }
        }

        fn new(&self, route: Route) -> Result<Route, Error> {
            let conn = &mut get_connection_v2!(self.db);
            let trx_result = conn.transaction::<i64, diesel::result::Error, _>(|tx_conn| {
                let entity = InsertRouteDB {
                    start: route.start.clone(),
                    finish:route.finish.clone(),
                    price: route.price.clone(),
                };
                match diesel::insert_into(route_dsl::routes)
                    .values(&entity)
                    .execute(tx_conn) {
                        Err(err) => Err(err),
                        Ok(_) => {
                            match route_dsl::routes
                                .select(last_insert_id())
                                .load::<i64>(tx_conn) {
                                    Err(err) => Err(err),
                                    Ok(ids) if ids.len() > 0 => Ok(ids[0].clone()),
                                    _ => Ok(-1),
                                }
                        },
                    }
            });
            match trx_result {
                Ok(id) => Ok(Route {
                    id: id,
                    start: route.start.clone(),
                    finish: route.finish.clone(),
                    price: route.price.clone(),
                }),
                Err(err) => Err(Error::internal(DbSave, err.to_string())),
            }
        }

        fn update(&self, route: Route) -> Result<(), Error> {
            let conn = &mut get_connection_v2!(self.db);
            match diesel::update(route_dsl::routes)
                .filter(route_dsl::id.eq(route.id.clone()))
                .set((
                    route_dsl::start.eq(route.start.clone()),
                    route_dsl::finish.eq(route.finish.clone()),
                    route_dsl::price.eq(route.price.clone())
                ))
                .execute(conn) {
                    Err(err) => Err(Error::internal(DbSave, err.to_string())),
                    Ok(affected) if affected <= 0 => Err(Error::not_found("route not found".to_string())),
                    _ => Ok(()),
                }
        }

        fn delete(&self, id: i64) -> Result<(), Error> {
            let conn = &mut get_connection_v2!(self.db);
            match diesel::delete(
                route_dsl::routes
                    .filter(route_dsl::id.eq(id))
            ).execute(conn)  {
                Err(err) => Err(Error::internal(DbSave, err.to_string())),
                Ok(result) if result > 0 => Ok(()),
                _ => Err(Error::not_found("route not found".to_string())),
            }
        }

        fn find_by_start(&self, start: i64, exclude_finishes: Option<Vec<i64>>) -> Result<Vec<Route>, Error> {
            let conn = &mut get_connection_v2!(self.db);
            let mut builder = route_dsl::routes.into_boxed();
            builder = builder.filter(route_dsl::start.eq(start.clone()));
            if exclude_finishes.is_some() {
                builder = builder.filter(route_dsl::finish.ne_all(exclude_finishes.unwrap()));
            }
            match builder.select(RouteDB::as_select()).load(conn) {
                Ok(result) => Ok(result.iter().map(|r| r.to_model()).collect()),
                Err(err) => Err(Error::internal(DbRead, err.to_string())),
            }
        }
    }
}
