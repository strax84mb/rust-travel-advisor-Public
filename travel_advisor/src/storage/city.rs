pub mod cities {
    use std::sync::Arc;

    use async_trait::async_trait;
    use sqlx::FromRow;

    use crate::{
        model::City,
        util::app_errors::Error,
    };
    use super::super::Database;

    #[async_trait]
    pub trait CityRepository {
        async fn get_all(&self) -> Result<Vec<City>, Error>;
        async fn get_by_id(&self, id: i64) -> Result<Option<City>, Error>;
        async fn new(&self, name: String) -> Result<City, Error>;
        async fn get_by_name(&self, name: String) -> Result<Option<City>, Error>;
    }

    struct CityRepositoryImpl {
        db: Arc<Database>,
    }

    pub fn new_city_repository(db: Arc<Database>) -> Arc<impl CityRepository> {
        Arc::new(CityRepositoryImpl {
            db: db,
        })
    }

    #[async_trait]
    impl CityRepository for CityRepositoryImpl {

        async fn get_all(&self) ->  Result<Vec<City> ,Error> {
            let pool = self.db.connections.as_ref();
            match sqlx::query("SELECT id, name FROM cities").fetch_all(pool).await {
                Ok(rows) => {
                    let result: Result<Vec<City>, sqlx::Error> = rows.iter().map(|row| City::from_row(row)).collect();
                    match result {
                        Ok(v) => Ok(v),
                        Err(err) => Err(Error::underlying(err.to_string()))
                    }
                },
                Err(err) => Err(Error::underlying(err.to_string()))
            }
        }

        async fn get_by_id(&self, id: i64) -> Result<Option<City>, Error> {
            let pool = self.db.connections.as_ref();
            let result = sqlx::query("SELECT id, name FROM cities WHERE id = ?")
                .bind(id)
                .fetch_optional(pool)
                .await;
            match result {
                Ok(row) => match row {
                    Some(row) => match City::from_row(&row) {
                        Ok(city) => Ok(Some(city)),
                        Err(err) => Err(Error::underlying(err.to_string())),
                    },
                    None => Ok(None),
                },
                Err(sqlx::Error::RowNotFound) => Err(Error::not_found()),
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }

        async fn new(&self, name: String) -> Result<City, Error> {
            let pool = self.db.connections.as_ref();
            let result = sqlx::query("INSERT INTO cities (name) VALUES (?)")
                .bind(name.clone())
                .execute(pool)
                .await;
            match result {
                Ok(row) => {
                    if row.rows_affected() == 0 {
                        return Err(Error::underlying("no rows inserted".to_string()));
                    }

                    let id = row.last_insert_id() as i64;

                    Ok(City::new(id, name))
                },
                Err(err) => Err(Error::underlying(err.to_string())),
            }
        }

        async fn get_by_name(&self, name: String) -> Result<Option<City>, Error> {
            let pool = self.db.connections.as_ref();
            let statement = sqlx::query(
                "SELECT id, name FROM cities WHERE LOWER(name) = ?"
                ).bind(name.clone().to_lowercase())
                .fetch_optional(pool);
            match statement.await {
                Ok(row) => match row {
                    Some(row) => match City::from_row(&row) {
                        Ok(city) => Ok(Some(city)),
                        Err(err) => Err(Error::underlying(format!("failed to read row: {}", err.to_string()))),
                    }, 
                    None => Ok(None),
                },
                Err(err) => Err(Error::underlying(format!("failed execute query: {}", err.to_string()))),
            }
        }

    }

}