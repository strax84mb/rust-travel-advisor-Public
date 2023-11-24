use actix_web::http::header::ToStrError;
use async_trait::async_trait;

use crate::{
    util::app_errors::Error,
    model::{
        Airport,
        City,
        Comment,
        User,
    },
};

use super::UserData;

#[async_trait]
pub trait CityService {
    async fn get_all(&self) -> Result<Vec<City>, Error>;
    async fn get_full(&self, id: i64) -> Result<Option<City>, Error>;
    async fn save_cities(&self, sv_text: &[u8]) -> Result<(), Error>;
}

#[async_trait]
pub trait AirportService {
    async fn get_all(&self) -> Result<Vec<Airport>, Error>;
    async fn get_by_id(&self, id: i64) -> Result<Option<Airport>, Error>;
    async fn create(&self, airport: Airport) -> Result<Airport, Error>;
    async fn update(&self, airport: Airport) -> Result<(), Error>;
    async fn delete(&self, id: i64) -> Result<(), Error>;
    async fn save_airports(&self, sv_text: &[u8]) -> Result<(), Error>;
}

#[async_trait]
pub trait CommentService {
    async fn create(&self, user_id: i64, mut comment: Comment) -> Result<Comment, Error>;
    async fn update(&self, user_id: i64, comment: Comment) -> Result<Comment, Error>;
    async fn delete(&self, id: i64, user: User) -> Result<(), Error>;
    async fn list_for_city(&self, city_id: i64) -> Result<Vec<Comment>, Error>;
    async fn list_for_user(&self, user_id: i64) -> Result<Vec<Comment>, Error>;
    async fn get_by_id(&self, id: i64) -> Result<Option<Comment>, Error>;
}

#[async_trait]
pub trait AuthService {
    async fn create_jwt(&self, user: User) -> Result<UserData, Error>;
    async fn get_user(&self, header: Option<Result<&str, ToStrError>>) -> Result<User, Error>;
    async fn has_role(&self, header: Option<Result<&str, ToStrError>>, roles: Vec<&str>) -> Result<bool, Error>;
}