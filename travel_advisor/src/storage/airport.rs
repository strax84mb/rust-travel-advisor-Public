pub mod airports {
    use std::sync::Arc;

    use async_trait::async_trait;
    use sqlx::{
        FromRow,
        Row,
    };

    use crate::{
        model::Airport,
        util::app_errors::Error, storage::Database,
    };

    #[async_trait]
    pub trait AirportRepository {
        async fn get_all(&self) -> Result<Vec<Airport>, Error>;
        async fn get_by_id(&self, id: i64) -> Result<Option<Airport>, Error>;
        async fn new(&self, airport: &Airport) -> Result<Airport, Error>;
        async fn update(&self, airport: Airport) -> Result<(), Error>;
        async fn delete(&self, id: i64) -> Result<(), Error>;
        async fn get_by_city_id(&self, city_id: i64) -> Result<Vec<Airport>, Error>;
    }

    struct AirportRepositoryImpl {
        db: Arc<Database>,
    }

    pub fn new_airport_repository(db: Arc<Database>) -> Arc<impl AirportRepository> {
        Arc::new(AirportRepositoryImpl {
            db: db,
        })
    }

    #[async_trait]
    impl AirportRepository for AirportRepositoryImpl {

        async fn get_all(&self) -> Result<Vec<Airport>, Error> {
            let pool = self.db.connections.as_ref();
            match sqlx::query("SELECT id, city_id, `name` FROM airports").fetch_all(pool).await {
                Ok(rows) => {
                    let result: Result<Vec<Airport>, sqlx::Error> = rows.iter().map(|r| Airport::from_row(r)).collect();
                    match result {
                        Ok(airports) => Ok(airports),
                        Err(err) => Err(Error::underlying(err.to_string())),
                    }
                },
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }

        async fn get_by_id(&self, id: i64) -> Result<Option<Airport>, Error> {
            let pool = self.db.connections.as_ref();
            let result = sqlx::query("SELECT id, city_id, `name` FROM airports WHERE id = ?")
                .bind(id)    
                .fetch_optional(pool)
                .await;
            match result {
                Ok(row) => match row {
                    Some(row) => match Airport::from_row(&row) {
                        Ok(airport) => Ok(Some(airport)),
                        Err(err) => Err(Error::underlying(err.to_string())),
                    },
                    None => Ok(None),
                }
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }

        async fn new(&self, airport: &Airport) -> Result<Airport, Error> {
            let pool = self.db.connections.as_ref();
            let mut tx = match pool.begin().await {
                Ok(t) => t,
                Err(err) => return Err(Error::underlying(err.to_string())),
            };
    
            let statement = sqlx::query(
                    "INSERT INTO airports (city_id, `name`) VALUES (?, ?)"
                )
                .bind(airport.city_id)
                .bind(airport.name.clone())
                .execute(&mut tx);
            
            let mut result: Result<i64, Error> = match statement.await {
                Ok(row) => {
                    if row.rows_affected() == 0 {
                        Err(Error::underlying("No row inserted".to_string()))
                    } else {
                        match sqlx::query("SELECT LAST_INSERT_ID()").fetch_one(&mut tx).await {
                            Ok(row) => {
                                let id: i64 = match row.try_get(0) {
                                    Ok(v) => v,
                                    Err(err) => return Err(Error::underlying(err.to_string())),
                                };
    
                                Ok(id)
                            },
                            Err(err) => Err(Error::underlying(err.to_string())),
                        }
                    }
                },
                Err(err) => Err(Error::underlying(err.to_string())),
            };
    
            let mut final_airport = airport.clone();
            if result.is_ok() {
                final_airport.id = result.unwrap();
    
                result = match tx.commit().await {
                    Ok(_) => Ok(0),
                    Err(err) => Err(Error::underlying(err.to_string())),
                }
            }
    
            match result {
                Ok(_) => Ok(final_airport),
                Err(err) => Err(err),
            }
        }

        async fn update(&self, airport: Airport) -> Result<(), Error> {
            let pool = self.db.connections.as_ref();
            let statement = sqlx::query("UPDATE airports SET city_id = ?, `name` = ? WHERE id = ?")
                .bind(airport.city_id)
                .bind(airport.name)
                .bind(airport.id)
                .execute(pool)
                .await;
            match statement {
                Ok(result) => {
                    if result.rows_affected() == 0 {
                        return Err(Error::not_found());
                    }
    
                    Ok(())
                },
                Err(sqlx::Error::RowNotFound) => Err(Error::not_found()),
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }

        async fn delete(&self, id: i64) -> Result<(), Error> {
            let pool = self.db.connections.as_ref();
            let statement = sqlx::query("DELETE FROM airports WHERE id = ?")
                .bind(id)
                .execute(pool)
                .await;
            match statement {
                Ok(result) => {
                    if result.rows_affected() == 0 {
                        Err(Error::not_found())
                    } else {
                        Ok(())
                    }
                },
                Err(sqlx::Error::RowNotFound) => Err(Error::not_found()),
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }

        async fn get_by_city_id(&self, city_id: i64) -> Result<Vec<Airport>, Error> {
            let pool = self.db.connections.as_ref();
            let statement = sqlx::query("SELECT id, city_id, `name` FROM airports WHERE city_id = ?")
                .bind(city_id)
                .fetch_all(pool);
            match statement.await {
                Ok(rows) => {
                    let result: Result<Vec<Airport>, sqlx::Error> = rows.iter().map(|r| Airport::from_row(r)).collect();
                    match result {
                        Ok(airports) => Ok(airports),
                        Err(err) => Err(Error::underlying(err.to_string())),
                    }
                },
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }

    }

}