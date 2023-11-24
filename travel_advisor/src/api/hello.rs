use std::sync::Arc;

use actix_web::{get, web::{self, Data}, Responder, HttpResponse};

use crate::services::traits::CityService;

 
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world)
        .service(take_test_for_a_ride);
}

#[get("/v1/hello")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("World!")
}

#[get("/v1/hello/test-traits")]
async fn take_test_for_a_ride(city_service: Data<Arc<dyn CityService + Send + Sync>>) -> impl Responder {
    match city_service.into_inner().get_all().await {
        Ok(result) => println!(">>> count: {}", result.len()),
        Err(err) => println!(">>> error: {}", err.to_string()),
    };
    HttpResponse::Ok().finish()
}