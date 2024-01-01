mod config;
pub mod api;
pub mod model;
pub mod services;
mod storage;
pub mod util;
mod playground;
pub mod schema;
mod middleware;

use std::{
    process::exit,
    sync::Arc,
};
use log::{
    info,
    error,
};

use actix_web::{
    App,
    HttpServer,
    web::Data,
};

use crate::{
    config::Config,
    middleware::RequestId,
    services::{
        new_airport_service,
        new_auth_service,
        new_city_service,
        new_comment_service,
        new_route_service,
        traits::{
            AirportService,
            AuthService,
            CityService,
            CommentService,
            RouteService,
        },
    },
    storage::{
        Database,
        AirportRepository,
        CityRepository,
        CommentRepository,
        UserRepository,
        routes::RouteRepository,
        new_airport_repository,
        new_city_repository,
        new_comment_repository,
        new_user_repository,
        routes::new_route_repository,
    },
};


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug)
            .chan_len(Some(100000))
    ).unwrap();

    let config_file: &'static str = "config.yml";
    let config = Config::from_file(config_file);
    info!("Using configuration file from {0}", config_file);


    let database = Database::new(config.get_database_url().clone());
    let database = match database.await {
        Ok(db) => db,
        Err(err) => {
            error!("Failed to init DB: {}", err.as_str());
            exit(1);
        },
    };

    let db_arc = Arc::new(database);

    let airport_repo: Arc<dyn AirportRepository + Sync + Send> = new_airport_repository(db_arc.clone());
    let city_repo: Arc<dyn CityRepository + Sync + Send> = new_city_repository(db_arc.clone());
    let comment_repo: Arc<dyn CommentRepository + Sync + Send> = new_comment_repository(db_arc.clone());
    let user_repo: Arc<dyn UserRepository + Sync + Send> = new_user_repository(db_arc.clone());
    let route_repo: Arc<dyn RouteRepository + Sync + Send> = new_route_repository(db_arc.clone());

    let auth_service = new_auth_service(config.key(), user_repo.clone()).expect("could not instantiate auth service");
    let auth_service_data: Data<Arc<dyn AuthService + Send + Sync>> = Data::new(auth_service.clone());

    let airport_service = new_airport_service(city_repo.clone(), airport_repo.clone());
    let airport_service_data: Data<Arc<dyn AirportService + Send + Sync>> = Data::new(airport_service.clone());

    let city_service = new_city_service(city_repo.clone(), airport_repo.clone());
    let city_service_data: Data<Arc<dyn CityService + Send + Sync>> = Data::new(city_service.clone());

    let comment_service = new_comment_service(comment_repo.clone());
    let comment_service_data: Data<Arc<dyn CommentService + Send + Sync>> = Data::new(comment_service.clone());

    let route_service = new_route_service(
        route_repo.clone(),
        airport_repo.clone(),
        city_repo.clone()
    );
    let route_service_data: Data<Arc<dyn RouteService + Send + Sync>> = Data::new(route_service.clone());

    let user_repo_data: Data<Arc<dyn UserRepository + Send + Sync>> = Data::new(user_repo.clone());

    //let jwt_extractor = new_jwt_extractor(auth_service.clone());

    let app = HttpServer::new(move || {
        App::new()
            .app_data(airport_service_data.clone())
            .app_data(auth_service_data.clone())
            .app_data(city_service_data.clone())
            .app_data(comment_service_data.clone())
            .app_data(user_repo_data.clone())
            .app_data(route_service_data.clone())
            .wrap(RequestId)
            //.wrap(jwt_extractor)
            .configure(crate::api::init_hello)
            .configure(crate::api::init_cities)
            .configure(crate::api::init_users)
            .configure(crate::api::init_airports)
            .configure(crate::api::init_comments)
            .configure(crate::api::init_routes)
        }
    ).bind(config.get_app_url())?;

    app.run().await
}
