use std::sync::Arc;

use actix_web::{
    get,
    post,
    web::{
        self,
        Data,
    },
    Responder,
    HttpRequest,
};
use test_annotations::roles;

use crate::{
    AuthService,
    CityService,
};
use super::{
    dtos::CityDto,
    validations::{
        extract_auth,
        string_to_id,
    },
    responses::{
        resolve_error,
        respond_bad_request,
        respond_not_found,
        respond_unauthorized,
        respond_ok,
    },
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cities)
        .service(get_city_by_id)
        .service(upload_cities);
}

#[get("/v1/cities")]
async fn get_cities(city_service: Data<Arc<dyn CityService + Send + Sync>>) -> impl Responder {
    // load cities
    let result = match city_service.into_inner().get_all() {
        Ok(cities) => cities,
        Err(err) => return resolve_error(err, None),
    };
    // convert to DTOs
    let dtos: Vec<CityDto> = result.iter()
        .map(|c| CityDto::from_model(c))
        .collect();

    respond_ok(Some(dtos))
}

#[get("/v1/cities/{id}")]
async fn  get_city_by_id(
    id: web::Path<String>,
    city_service: Data<Arc<dyn CityService + Send + Sync>>,
) -> impl Responder {
    // get id
    let city_id = match string_to_id(id.to_string()) {
        Ok(v) => v,
        Err(err) => return respond_bad_request(format!("failed to parse city ID: {}", err.to_string())),
    };
    // load city
    let city = match city_service.into_inner().get_full(city_id) {
        Ok(city) => match city {
            Some(city) => CityDto::from_model(&city),
            None => return respond_not_found("city not found"),
        },
        Err(err) => return resolve_error(err, Some("failed to load city")),
    };

    respond_ok(Some(city))
}

#[post("/v1/cities")]
#[roles("admin")]
async fn upload_cities(
    payload: web::Bytes,
    city_service: Data<Arc<dyn CityService + Send + Sync>>,
) -> impl Responder {
    match city_service.into_inner().save_cities(payload.to_vec().as_slice()) {
        Ok(()) => respond_ok(Some("saved all cities")),
        Err(err) => resolve_error(err, Some("failed to save all cities")),
    }
}