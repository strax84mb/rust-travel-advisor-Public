use std::time::SystemTime;

use serde::{
    Serialize,
    Deserialize,
};

use crate::model::{
    Airport,
    City,
    Comment,
    Route,
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

#[derive(Serialize)]
pub struct AirportStopDto {
    id: i64,
    name: String,
    route_to_next: Option<RouteDto>,
}

#[derive(Serialize)]
pub struct BestRouteDto {
    stops: Vec<AirportStopDto>,
}
