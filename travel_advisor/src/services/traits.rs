use actix_web::http::header::ToStrError;

use crate::{
    util::Error,
    model::{
        Airport,
        City,
        Comment,
        Route,
        User,
    },
};

use super::UserData;

pub trait CityService {
    fn get_all(&self) -> Result<Vec<City>, Error>;
    fn get_full(&self, id: i64) -> Result<Option<City>, Error>;
    fn new(&self, name: String) -> Result<City, Error>;
    fn save_cities(&self, sv_text: &[u8]) -> Result<(), Error>;
}

pub trait AirportService {
    fn get_all(&self) -> Result<Vec<Airport>, Error>;
    fn get_by_id(&self, id: i64) -> Result<Option<Airport>, Error>;
    fn create(&self, airport: Airport) -> Result<Airport, Error>;
    fn update(&self, airport: Airport) -> Result<(), Error>;
    fn delete(&self, id: i64) -> Result<(), Error>;
    fn save_airports(&self, sv_text: &[u8]) -> Result<(), Error>;
}

pub trait CommentService {
    fn create(&self, user_id: i64, comment: Comment) -> Result<Comment, Error>;
    fn update(&self, user_id: i64, comment: Comment) -> Result<Comment, Error>;
    fn delete(&self, id: i64, user: User) -> Result<(), Error>;
    fn list_for_city(&self, city_id: i64) -> Result<Vec<Comment>, Error>;
    fn list_for_user(&self, user_id: i64) -> Result<Vec<Comment>, Error>;
    fn get_by_id(&self, id: i64) -> Result<Option<Comment>, Error>;
}

pub trait AuthService {
    fn create_jwt(&self, user: User) -> Result<UserData, Error>;
    fn get_user(&self, header: Option<Result<&str, ToStrError>>) -> Result<User, Error>;
    fn get_user_if_has_role(&self, header: Option<Result<&str, ToStrError>>, roles: Vec<&str>) -> Result<Option<User>, Error>;
}

pub trait RouteService {
    fn get_all(&self, offset: i64, limit: i64) -> Result<Vec<Route>, Error>;
    fn find_by_id(&self, id: i64) -> Result<Option<Route>, Error>;
    fn save_routes(&self, sv_text: &[u8]) -> Result<(), Error>;
    fn update(&self, route: Route) -> Result<(), Error>;
    fn delete(&self, id: i64) -> Result<(), Error>;
    fn find_cheapest_route(&self, start: i64, finish: i64) -> Result<Vec<Route>, Error>;
}