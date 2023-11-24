mod auth;
mod airport_service;
mod city_service;
mod comment_service;
pub mod traits;

pub type UserData = auth::services::UserData;

pub use airport_service::services::new_airport_service as new_airport_service;
pub use auth::services::new_auth_service as new_auth_service;
pub use city_service::services::new_city_service as new_city_service;
pub use comment_service::services::new_comment_service as new_comment_service;

mod comment_service_test;
