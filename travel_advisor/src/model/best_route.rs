use std::sync::Arc;

use crate::{
    util::Error,
    storage::{
        routes::RouteRepository,
        AirportRepository,
    },
};
use super::Route;

pub struct BestRoute {
    lowest_price: Option<i64>,
    best_path: Option<Vec<i64>>,
    expanded_during_this_turn: bool,
    destination: Vec<i64>,
    route_repo: Arc<dyn RouteRepository + Sync + Send>,
    airport_repo: Arc<dyn AirportRepository + Sync + Send>,
}

pub struct Stop {
    accumulated_price: i64,
    incomming_route: Option<Box<Route>>,
    destinations: Option<Vec<Box<Stop>>>,
    dead_end: bool,
}

pub struct DoExpandInput {
    previous_stops: Box<Vec<i64>>,
    accumulated_price: i64,
}

impl BestRoute {
    pub fn new(
        start_id: i64,
        destination: Vec<i64>,
        route_repo: Arc<dyn RouteRepository + Sync + Send>,
        airport_repo: Arc<dyn AirportRepository + Sync + Send>,
    ) -> (BestRoute, Stop) {
        (BestRoute {
            lowest_price: None,
            best_path: None,
            destination: destination,
            expanded_during_this_turn: false,
            route_repo: route_repo,
            airport_repo: airport_repo,
        }, Stop {
            accumulated_price: 0,
            dead_end: false,
            destinations: None,
            incomming_route: Some(Box::new(Route{
                id: 0,
                price: 0,
                start: 0,
                finish: start_id,
            })),
        })
    }

    pub fn search_for_best_path(&mut self, root: &mut Stop) -> Result<Option<Vec<i64>>, Error> {
        let mut keep_searching = true;
        while keep_searching {
            self.expanded_during_this_turn = false;
            let start_id = root.incomming_route.as_ref().unwrap().finish.clone();
            match self.do_expand(root, DoExpandInput {
                accumulated_price: 0,
                previous_stops: Box::new(vec![start_id]),
            }) {
                Ok(()) => keep_searching = self.expanded_during_this_turn,
                Err(err) => return Err(err),
            };
        }
        match self.best_path.as_ref() {
            Some(q) => Ok(Some(q.iter().map(|id| id.clone()).collect())),
            None => Ok(None),
        }
    }

    fn do_expand(&mut self, 
        stop: &mut Stop,
        input: DoExpandInput,
    ) -> Result<(), Error> {
        if stop.dead_end {
            return Ok(());
        }
        if (self.lowest_price.is_none() || self.lowest_price.as_ref().unwrap().clone() > stop.accumulated_price) 
            && stop.incomming_route.is_some() 
            && self.destination.contains(&(stop.incomming_route.as_ref().unwrap().finish.clone())) {
                self.lowest_price = Some(stop.accumulated_price.clone());
                self.destination = input.previous_stops.as_ref().iter().map(|id| id.clone()).collect();
                stop.dead_end = true;
                return Ok(());
            }
        if self.lowest_price.is_none() || self.lowest_price.as_ref().unwrap().clone() < stop.accumulated_price {
            stop.dead_end = true;
            return Ok(());
        }
        if stop.dead_end {
            return Ok(());
        } else {
            if stop.destinations.is_none() {
                let path_so_far = input.previous_stops.as_ref().iter().map(|id| id.clone()).collect();
                let start = stop.incomming_route.as_ref().unwrap().finish.clone();
                let start_points = match self.extend_id_vec(vec![start]) {
                    Ok(ids) => ids,
                    Err(err) => return Err(err),
                };
                let routes = match self.get_possible_destinations(start_points, path_so_far) {
                    Ok(r) => r,
                    Err(err) => return Err(err),
                };
                if routes.len() == 0 {
                    stop.dead_end = true;
                    return Ok(());
                }
                let mut destinations: Vec<Box<Stop>> = vec![];
                for route in routes {
                    destinations.push(Box::new(Stop {
                        dead_end: false,
                        destinations: None,
                        accumulated_price: stop.accumulated_price.clone() + route.price.clone(),
                        incomming_route: Some(Box::new(route)),
                    }));
                }
                stop.destinations = Some(destinations);
                self.expanded_during_this_turn = true;
            } else {
                for s in stop.destinations.as_mut().unwrap() {
                    let mut previous_stops: Vec<i64> = input.previous_stops.as_ref().iter().map(|id| id.clone()).collect();
                    previous_stops.push(s.incomming_route.as_ref().unwrap().finish.clone());
                    match self.do_expand(
                        s.as_mut(),
                        DoExpandInput {
                            accumulated_price: input.accumulated_price.clone(),
                            previous_stops: Box::new(previous_stops),
                        },
                    ) {
                        Ok(()) => (),
                        Err(err) => return Err(err),
                    };
                }
            }
        }
        Ok(())
    }

    fn get_possible_destinations(&self, start_points: Vec<i64>, ids_to_exclude: Vec<i64>) -> Result<Vec<Route>, Error> {
        self.route_repo.find_by_start(start_points, Some(ids_to_exclude))
    }

    fn extend_id_vec(&self, route_starts: Vec<i64>) -> Result<Vec<i64>, Error> {
        self.airport_repo.get_ids_by_city_ids(route_starts)
    }
}


#[cfg(test)]
pub mod testing {
    struct NumContent {
        num: i32,
    }

    #[test]
    fn test_changing_boxed_contents() {
        let mut num = NumContent {
            num: 5,
        };
        let mut boxed = Box::new(num);
        {
            let refed = boxed.as_mut();
            refed.num += 1;
        }
        let q = boxed.as_ref();
        let addr = &mut boxed;
        {
            let w = addr.as_mut();
            w.num += 1;
        }
        let r = Box::new(addr.as_ref());
        addr.num;
        println!("{}", boxed.num.clone());
    }

}