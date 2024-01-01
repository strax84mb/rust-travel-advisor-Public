use std::time::SystemTime;

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    model::{
        Airport,
        City,
        Comment,
        Route,
    },
    util::Error,
};

pub trait ToModel<T> {
    fn to_model(&self) -> T;
}

pub trait FromModel<T> {
    fn from_model(model: &T) -> Self;
}

#[derive(Serialize)]
pub struct CityDto {
    pub id: i64,
    pub name: String,
    pub airports: Vec<AirportDto>,
}

impl FromModel<City> for CityDto {
    fn from_model(c: &City) -> Self {
        let airports: Vec<AirportDto> = c.airports.iter()
            .map(|a| AirportDto::from_model(a))
            .collect();
        
        CityDto {
            id: c.id,
            name: c.name.clone(),
            airports: airports,
        }
    }
}

#[derive(Serialize)]
pub struct UserDto {
    pub id: i64,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pass: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: i64,
    pub token: String,
}

#[derive(Deserialize)]
pub struct CreateAirportDto {
    pub city_id: i64,
    pub name: String,
}

impl ToModel<Airport> for CreateAirportDto {
    fn to_model(&self) -> Airport {
        Airport {
            id: 0,
            city_id: self.city_id.clone(),
            name: self.name.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct AirportDto {
    pub id: i64,
    pub city_id: i64,
    pub name: String,
}

impl FromModel<Airport> for AirportDto {
    fn from_model(a: &Airport) -> Self {
        AirportDto {
            id: a.id.clone(),
            city_id: a.city_id.clone(),
            name: a.name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommentDto {
    pub id: i64,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    pub city_id: i64,
    pub content: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl CommentDto {
    pub fn from_model(c: &Comment) -> Self {
        CommentDto {
            id: c.id.clone(),
            user_id: c.user_id.clone(),
            user_name: None,
            city_id: c.city_id.clone(),
            content: c.content.clone(),
            created_at: c.created_at.clone(),
            updated_at: c.updated_at.clone(),
        }
    }

    pub fn to_model(&self) -> Comment {
        Comment {
            id: self.id.clone(),
            user_id: self.user_id.clone(),
            city_id: self.city_id.clone(),
            content: self.content.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct RouteDto {
    pub id: i64,
    pub start: i64,
    pub finish: i64,
    pub price: i64,
}

impl FromModel<Route> for RouteDto {
    fn from_model(model: &Route) -> Self {
        RouteDto {
            id: model.id.clone(),
            start: model.start.clone(),
            finish: model.finish.clone(),
            price: model.price.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct SaveRouteDto {
    pub start: i64,
    pub finish: i64,
    pub price: i64,
}

#[derive(Deserialize)]
pub struct CalculateCheapestRouteRequestDto {
    pub starting_city_id: i64,
    pub destination_city_id: i64,
}

#[derive(Serialize)]
pub struct AirportStopDto {
    id: i64,
    name: String,
    route_to_next: Option<RouteDto>,
}

#[derive(Deserialize)]
pub struct PaginationQueryParam {
    pub offset: Option<String>,
    pub limit: Option<String>,
}

#[derive(Serialize)]
pub struct BestPathDto {
    steps: Vec<PathStepDto>,
}

#[derive(Serialize)]
pub struct PathStepDto {
    city_id: i64,
    city_name: String,
    airport_id: i64,
    airport_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<i64>,
    step_type: PathStepType,
}

#[derive(Serialize)]
#[serde(rename_all="kebab-case")]
pub enum PathStepType {
    Start,
    Flight,
    CityCommute,
}

impl BestPathDto {
    pub fn from_models(
        routes: Vec<Route>,
        airports: Vec<Airport>,
        cities: Vec<City>,
    ) -> Result<BestPathDto, Error> {
        let mut path = BestPathDto {
            steps: vec![],
        };
        let mut prev_route = &routes[0];
        let airport_id = routes[0].start.clone();
        let mut prev_airport = match airports.iter().find(|a| a.id.clone() == airport_id.clone()) {
            Some(opt) => opt,
            None => return Err(Error::not_found(format!("airport with ID={} not found", airport_id.clone()))),
        };
        let mut prev_city = match cities.iter().find(|c| c.id.clone() == prev_airport.city_id.clone()) {
            Some(opt) => opt,
            None => return Err(Error::not_found(format!("city with ID={} not found", prev_airport.city_id.clone()))),
        };
        // start of path
        path.steps.push(PathStepDto {
            airport_id: airport_id.clone(),
            airport_name: prev_airport.name.clone(),
            city_id: prev_city.id.clone(),
            city_name: prev_city.name.clone(),
            price: None,
            route_id: None,
            step_type: PathStepType::Start,
        });
        prev_airport = match airports.iter().find(|a| a.id.clone() == prev_route.finish.clone()) {
            Some(opt) => opt,
            None => return Err(Error::not_found(format!("airport with ID={} not found", airport_id.clone()))),
        };
        prev_city = match cities.iter().find(|c| c.id.clone() == prev_airport.city_id.clone()) {
            Some(opt) => opt,
            None => return Err(Error::not_found(format!("city with ID={} not found", prev_airport.city_id.clone()))),
        };
        path.steps.push(PathStepDto {
            airport_id: prev_airport.id.clone(),
            airport_name: prev_airport.name.clone(),
            city_id: prev_city.id.clone(),
            city_name: prev_city.name.clone(),
            price: Some(prev_route.price.clone()),
            route_id: Some(prev_route.id.clone()),
            step_type: PathStepType::Flight,
        });
        // rest of steps
        for route in routes.iter().skip(1) {
            if route.start.clone() != prev_route.finish.clone() {
                prev_airport = match airports.iter().find(|a| a.id.clone() == route.start.clone()) {
                    Some(opt) => opt,
                    None => return Err(Error::not_found(format!("airport with ID={} not found", route.start.clone()))),
                };
                prev_city = match cities.iter().find(|c| c.id.clone() == prev_airport.city_id.clone()) {
                    Some(opt) => opt,
                    None => return Err(Error::not_found(format!("city with ID={} not found", prev_airport.city_id.clone()))),
                };
                path.steps.push(PathStepDto {
                    airport_id: route.start.clone(),
                    airport_name: prev_airport.name.clone(),
                    city_id: prev_city.id.clone(),
                    city_name: prev_city.name.clone(),
                    route_id: None,
                    price: None,
                    step_type: PathStepType::CityCommute,
                });
            }
            prev_airport = match airports.iter().find(|a| a.id.clone() == route.finish.clone()) {
                Some(opt) => opt,
                None => return Err(Error::not_found(format!("airport with ID={} not found", route.finish.clone()))),
            };
            prev_city = match cities.iter().find(|c| c.id.clone() == prev_airport.city_id.clone()) {
                Some(opt) => opt,
                None => return Err(Error::not_found(format!("city with ID={} not found", prev_airport.city_id.clone()))),
            };
            path.steps.push(PathStepDto {
                airport_id: prev_airport.id.clone(),
                airport_name: prev_airport.name.clone(),
                city_id: prev_city.id.clone(),
                city_name: prev_city.name.clone(),
                route_id: Some(route.id.clone()),
                price: Some(route.price),
                step_type: PathStepType::Flight,
            });
            prev_route = route;
        }

        Ok(path)
    }
}
