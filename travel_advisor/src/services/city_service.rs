pub mod services {
    use std::sync::Arc;

    use log::error;
    use async_trait::async_trait;
    
    use crate::{
        AirportRepository,
        CityRepository,
        model::City,
        services::traits::CityService,
        util::app_errors::Error,
    };

    pub struct CityServiceImpl {
        city_repo: Arc<dyn CityRepository + Sync + Send>,
        airport_repo: Arc<dyn AirportRepository + Sync + Send>,
    }

    pub fn new_city_service(
        city_repo: Arc<dyn CityRepository + Sync + Send>,
        airport_repo: Arc<dyn AirportRepository + Sync + Send>,
    ) -> Arc<impl CityService> {
        Arc::new(CityServiceImpl {
            city_repo: city_repo,
            airport_repo: airport_repo,
        })
    }

    #[async_trait]
    impl CityService for CityServiceImpl {
        fn get_all(&self) -> Result<Vec<City>, Error> {
            self.city_repo.get_all()
        }

        fn get_full(&self, id: i64) -> Result<Option<City>, Error> {
            let mut city = match self.city_repo.get_by_id(id) {
                Ok(city) => match city {
                    Some(city) => city,
                    None => return Ok(None),
                },
                Err(err) => {
                    error!("failed to load city: {}", err.to_string());
                    return Err(Error::wrap_str(err, "failed to load city"));
                },
            };
            city.airports = match self.airport_repo.get_by_city_id(id) {
                Ok(a) => a,
                Err(err) => {
                    error!("failed to load airports: {}", err.to_string());
                    return Err(Error::wrap_str(err, "failed to load airports"));
                },
            };

            Ok(Some(city))
        }

        fn new(&self, name: String) -> Result<City, Error> {
            self.city_repo.new(name)
        }

        fn save_cities(&self, sv_text: &[u8]) -> Result<(), Error> {
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
        
                let name = record[0].to_string();
                match self.city_repo.new(name.clone()) {
                    Err(err) => {
                        error!("only pocessed {}: failed to save city: {}", count, err.to_string());
                        return Err(Error::wrap(err, format!("only pocessed {}: failed to save city", count)));
                    },
                    _ => (),
                };

                count += 1;
            };
        
            Ok(())
        }
    }
}
