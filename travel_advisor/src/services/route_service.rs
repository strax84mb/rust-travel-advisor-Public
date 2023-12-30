pub mod services {
    use std::sync::Arc;

    use log::error;

    use crate::{
        model::{
            common::FromStringRecord,
            Route,
            best_route::BestRoute,
        },
        storage::{
            routes::RouteRepository,
            AirportRepository,
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
    ) -> Arc<impl RouteService> {
        Arc::new(RouteServiceImpl {
            route_repo: route_repo,
            airport_repo: airport_repo,
        })
    }

    struct RouteServiceImpl {
        route_repo: Arc<dyn RouteRepository + Sync + Send>,
        airport_repo: Arc<dyn AirportRepository + Sync + Send>,
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

        fn find_cheapest_route(&self, start: i64, finish: i64) -> Result<Vec<Route>, Error> {
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
            let ex = self.extend_id_vec();
            let (mut best_route_finder, mut start) = BestRoute::new(
                start_airports[0].id.clone(),
                finish_airports.iter().map(|a| a.id.clone()).collect(),
                self.route_repo.clone(),
                self.airport_repo.clone(),
            );
            let q = self.extend_id_vec();
            let w = q(vec![]);
            best_route_finder.search_for_best_path(
                &mut start,
            );

            todo!("do this")
        }
    }

    impl RouteServiceImpl {
        fn get_possible_destinations(&self) -> Box<dyn Fn(Vec<i64>, Vec<i64>) -> Result<Vec<Route>, Error>> {
            Box::new(|start_points: Vec<i64>, ids_to_exclude| {
                Ok(vec![])
            })
        }

        fn extend_id_vec(&self) -> Box<dyn Fn(Vec<i64>) -> Result<Vec<i64>, Error> + '_> {
            Box::new(|ids| {
                let _q = self.airport_repo.get_by_city_id(1);
                Ok(vec![])
            })
        }
    }
}