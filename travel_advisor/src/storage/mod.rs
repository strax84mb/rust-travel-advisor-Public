mod db_context;
mod airport;
mod city;
mod user;
mod route;
mod comment;
mod entities;

pub type Database = db_context::Database;

pub use route::routes as routes;

pub use city::cities::new_city_repository as new_city_repository;
pub use city::cities::CityRepository as CityRepository;

pub use airport::airports::new_airport_repository as new_airport_repository;
pub use airport::airports::AirportRepository as AirportRepository;

pub use comment::comments::new_comment_repository as new_comment_repository;
pub use comment::comments::CommentRepository as CommentRepository;

pub use user::users::new_user_repository as new_user_repository;
pub use user::users::UserRepository as UserRepository;
