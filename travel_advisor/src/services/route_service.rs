pub mod services {
    use std::sync::Arc;

    use log::error;

    use crate::{
        model::{
            common::FromStringRecord,
            Route,
            best_route::BestRoute, Airport, City,
        },
        storage::{
            AirportRepository,
            CityRepository,
            routes::RouteRepository,
        },
        util::{
            Error,
            ErrorCode::TextRowParse,
        },
    };
    use super::super::{
        macros::log_if_error,
        traits::RouteService,
    };

    pub fn new_route_service(
        route_repo: Arc<dyn RouteRepository + Sync + Send>,
        airport_repo: Arc<dyn AirportRepository + Sync + Send>,
        city_repo: Arc<dyn CityRepository + Sync + Send>,
    ) -> Arc<impl RouteService> {
        Arc::new(RouteServiceImpl {
            route_repo: route_repo,
            airport_repo: airport_repo,
            city_repo: city_repo,
        })
    }

    struct RouteServiceImpl {
        route_repo: Arc<dyn RouteRepository + Sync + Send>,
        airport_repo: Arc<dyn AirportRepository + Sync + Send>,
        city_repo: Arc<dyn CityRepository + Sync + Send>,
    }

    impl RouteService for RouteServiceImpl {
        fn get_all(&self, offset: i64, limit: i64) -> Result<Vec<Route>, Error> {
            log_if_error!(self.route_repo.get_all(offset, limit))
        }

        fn find_by_id(&self, id: i64) -> Result<Option<Route>, Error> {
            log_if_error!(self.route_repo.find_by_id(id))
        }

        fn update(&self, route: Route) -> Result<(), Error> {
            log_if_error!(self.route_repo.update(route))
        }

        fn delete(&self, id: i64) -> Result<(), Error> {
            log_if_error!(self.route_repo.delete(id))
        }

        fn save_routes(&self, sv_text: &[u8]) -> Result<(), Error> {
            let mut count: i64 = 0;
            let mut csv_reader = csv::Reader::from_reader(sv_text);
            for record in csv_reader.records() {
                let record = match record {
                    Ok(r) => r,
                    Err(err) => {
                        error!("failed to get nex row. only pocessed {}: malformed CSV: {}", count, err.to_string());
                        return Err(Error::internal(TextRowParse, format!("only pocessed {}: malformed CSV: {}", count, err.to_string())));
                    },
                };
                let route = match Route::from_string_record(record) {
                    Ok(r) => r,
                    Err(err) => {
                        error!("failed to parse row. only pocessed {}: malformed CSV: {}", count, err.to_string());
                        return Err(Error::internal(TextRowParse, format!("only pocessed {}: malformed CSV: {}", count, err.to_string())));
                    },
                };
                match self.route_repo.new(route) {
                    Ok(_) => (),
                    Err(err) => {
                        error!("failed to save route. only pocessed {}: malformed CSV: {}", count, err.to_string());
                        return Err(Error::internal(TextRowParse, format!("only pocessed {}: malformed CSV: {}", count, err.to_string())));
                    },
                };
                count += 1;
            }
            Ok(())
        }

        fn find_cheapest_route(&self, start: i64, finish: i64) -> Result<(Vec<Route>, Vec<Airport>, Vec<City>), Error> {
            let start_airports = match self.airport_repo.get_by_city_id(start) {
                Ok(airports) => airports,
                Err(err) => {
                    error!("failed to load airports at starting city: {}", err.to_string());
                    return Err(err.wrap_str("failed to load airports at starting city"));
                },
            };
            let finish_airports = match self.airport_repo.get_by_city_id(finish) {
                Ok(airports) => airports,
                Err(err) => {
                    error!("failed to load airports at destination city: {}", err.to_string());
                    return Err(err.wrap_str("failed to load airports at destination city"));
                },
            };
            let (mut best_route_finder, mut start) = BestRoute::new(
                start_airports[0].id.clone(),
                finish_airports.iter().map(|a| a.id.clone()).collect(),
                self.route_repo.clone(),
                self.airport_repo.clone(),
            );
            let route_ids = match best_route_finder.search_for_best_path(&mut start) {
                Ok(opt) => match opt {
                    Some(ids) => ids,
                    None => return Err(Error::not_found("no route found".to_string())),
                },
                Err(err) => return Err(err.wrap_str("failed to calculate cheapest route")),
            };
            let routes = match self.route_repo.find_by_ids(route_ids.clone()) {
                Ok(routes) => routes,
                Err(err) => return Err(err.wrap(
                    format!("failed to get routes for IDs ({:?})", route_ids),
                )),
            };
            let mut airport_ids = routes.iter()
                .flat_map(|r| vec![r.start.clone(), r.finish.clone()])
                .collect::<Vec<i64>>();
            airport_ids.sort();
            airport_ids.dedup();
            let airports = match self.airport_repo.get_by_ids(airport_ids.clone()) {
                Ok(airports) => airports,
                Err(err) => return Err(err.wrap(format!(
                    "failed to get airports for IDs ({:?})", airport_ids
                ))),
            };
            let mut city_ids = airports.iter()
                .map(|a| a.city_id.clone())
                .collect::<Vec<i64>>();
            city_ids.sort();
            city_ids.dedup();
            let cities = match self.city_repo.get_by_ids(city_ids.clone()) {
                Ok(cities) => cities,
                Err(err) => return Err(err.wrap(format!(
                    "failed to get cities for IDs ({:?})", city_ids
                ))),
            };

            Ok((routes, airports, cities))
        }
    }
}