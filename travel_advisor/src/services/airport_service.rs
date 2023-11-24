pub mod services {
    use std::sync::Arc;

    use async_trait::async_trait;
    use log::error;

    use crate::{
        model::Airport,
        util::app_errors::Error,
        services::traits::AirportService,
        storage::{
            AirportRepository,
            CityRepository,
        },
    };

    pub fn new_airport_service(
        city_repo: Arc<dyn CityRepository + Sync + Send>,
        airport_repo: Arc<dyn AirportRepository + Sync + Send>,
    ) -> Arc<impl AirportService> {
        Arc::new(AirportServiceImpl {
            airport_repo: airport_repo,
            city_repo: city_repo,
        })
    }

    struct AirportServiceImpl {
        city_repo: Arc<dyn CityRepository + Sync + Send>,
        airport_repo: Arc<dyn AirportRepository + Sync + Send>,
    }

    #[async_trait]
    impl AirportService for AirportServiceImpl {

        async fn get_all(&self) -> Result<Vec<Airport>, Error> {
            self.airport_repo.get_all().await
        }

        async fn get_by_id(&self, id: i64) -> Result<Option<Airport>, Error> {
            self.airport_repo.get_by_id(id).await
        }

        async fn create(&self, airport: Airport) -> Result<Airport, Error> {
            self.airport_repo.new(&airport).await
        }

        async fn update(&self, airport: Airport) -> Result<(), Error> {
            self.airport_repo.update(airport).await
        }

        async fn delete(&self, id: i64) -> Result<(), Error> {
            self.airport_repo.delete(id).await
        }

        async fn save_airports(&self, sv_text: &[u8]) -> Result<(), Error> {
            let mut count: i64 = 0;
            let mut csv_reader = csv::Reader::from_reader(sv_text);
            for record in csv_reader.records() {
                let record = match record {
                    Ok(r) => r,
                    Err(err) => {
                        error!("only pocessed {}: malformed CSV: {}", count, err.to_string());
                        return Err(Error::underlying(format!("only pocessed {}: malformed CSV: {}", count, err.to_string())));
                    },
                };
                let city_name = record[0].to_string();
                let city = match self.city_repo.get_by_name(city_name.clone()).await {
                    Ok(city) => match city {
                        Some(city) => city,
                        None => {
                            error!("only pocessed {}: city {} not found", count, city_name);
                            return Err(Error::underlying(format!("only pocessed {}: city {} not found", count, city_name)));
                        },
                    },
                    Err(err) => {
                        error!("only pocessed {}: failed to load city {}: {}", count, city_name, err.to_string());
                        return Err(Error::underlying(format!("only pocessed {}: failed to load city {}: {}", count, city_name, err.to_string())));
                    },
                };
                let airport = Airport {
                    id: 0,
                    city_id: city.id,
                    name: record[1].to_string(),
                };
                match self.airport_repo.new(&airport).await {
                    Ok(_) => count += 1,
                    Err(err) => {
                        error!("only pocessed {}: failed to save airport: {}", count, err.to_string());
                        return Err(Error::underlying(format!("only pocessed {}: failed to save airport: {}", count, err.to_string())));
                    },
                }
            }

            Ok(())
        }

    }

}
