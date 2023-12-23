use std::sync::Arc;

use actix_web::{
    get,
    post,
    HttpResponse,
    Responder,
    web::{
        self,
        Data,
    },
};

use crate::services::traits::CityService;
use super::{
    dtos::CityDto,
    responses::{
        respond_ok,
        resolve_error,
    },
};
 
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/hello")
            .service(hello_world)
            .service(
                web::resource("/test-traits")
                    .guard(actix_web::guard::Get())
                    .to(take_test_for_a_ride)
            ).service(test_save_city)
    );
}

#[get("")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("World!")
}

async fn take_test_for_a_ride(city_service: Data<Arc<dyn CityService + Send + Sync>>) -> impl Responder {
    match city_service.into_inner().get_all() {
        Ok(result) => {
            println!(">>> count: {}", result.len());
            let dtos: Vec<CityDto> = result.iter().map(|c| CityDto::from_model(c)).collect();
            respond_ok(Some(dtos))
        },
        Err(err) => {
            println!(">>> error: {}", err.to_string());
            resolve_error(err, Some("failed it, ouchy!!!"))
        },
    }
}

#[post("/city/{name}")]
async fn test_save_city(
    name: web::Path<String>,
    city_service: Data<Arc<dyn CityService + Send + Sync>>,
) -> impl Responder {
    match city_service.new(name.to_string()) {
        Ok(city) => respond_ok(Some(CityDto::from_model(&city))),
        Err(err) => resolve_error(err, Some("failed to save city"))
    }
}