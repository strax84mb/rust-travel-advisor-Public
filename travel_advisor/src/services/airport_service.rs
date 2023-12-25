pub mod services {
    use std::sync::Arc;

    use log::error;

    use crate::{
        model::Airport,
        util::{
            Error,
            ErrorCode::{
                EntityNotFound,
                TextRowParse,
            },
        },
        services::traits::AirportService,
        storage::{
            AirportRepository,
    //        CityRepository,
        },
    };
    use crate::CityRepository;
    use super::super::macros::log_if_error;

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

    impl AirportService for AirportServiceImpl {

        fn get_all(&self) -> Result<Vec<Airport>, Error> {
            log_if_error!(self.airport_repo.get_all())
        }

        fn get_by_id(&self, id: i64) -> Result<Option<Airport>, Error> {
            log_if_error!(self.airport_repo.get_by_id(id))
        }

        fn create(&self, airport: Airport) -> Result<Airport, Error> {
            log_if_error!(self.airport_repo.new(&airport))
        }

        fn update(&self, airport: Airport) -> Result<(), Error> {
            log_if_error!(self.airport_repo.update(airport))
        }

        fn delete(&self, id: i64) -> Result<(), Error> {
            log_if_error!(self.airport_repo.delete(id))
        }

        fn save_airports(&self, sv_text: &[u8]) -> Result<(), Error> {
            let mut count: i64 = 0;
            let mut csv_reader = csv::Reader::from_reader(sv_text);
            for record in csv_reader.records() {
                let record = match record {
                    Ok(r) => r,
                    Err(err) => {
                        error!("only pocessed {}: malformed CSV: {}", count, err.to_string());
                        return Err(Error::internal(TextRowParse, format!("only pocessed {}: malformed CSV: {}", count, err.to_string())));
                    },
                };
                let city_name = record[0].to_string();
                let city = match self.city_repo.get_by_name(city_name.clone()) {
                    Ok(city) => match city {
                        Some(city) => city,
                        None => {
                            error!("only pocessed {}: city {} not found", count, city_name);
                            return Err(Error::internal(EntityNotFound, format!("only pocessed {}: city {} not found", count, city_name)));
                        },
                    },
                    Err(err) => {
                        error!("only pocessed {}: failed to load city {}: {}", count, city_name, err.to_string());
                        return Err(err.wrap(format!("only pocessed {}: failed to load city {}", count, city_name)));
                    },
                };
                let airport = Airport {
                    id: 0,
                    city_id: city.id,
                    name: record[1].to_string(),
                };
                match self.airport_repo.new(&airport) {
                    Ok(_) => count += 1,
                    Err(err) => {
                        error!("only pocessed {}: failed to save airport: {}", count, err.to_string());
                        return Err(err.wrap(format!("only pocessed {}: failed to save airport", count)));
                    },
                }
            }

            Ok(())
        }

    }

}
