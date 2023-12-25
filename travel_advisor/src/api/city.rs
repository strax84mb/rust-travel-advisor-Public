use std::sync::Arc;

use actix_web::{
    get,
    post,
    web::{
        self,
        Data,
    },
    Responder,
    HttpResponse, HttpRequest,
};

use crate::{
    AuthService,
    CityService,
    util::Error,
    
};
use super::{
    get_user_if_has_roles,
    dtos::CityDto,
    validations::string_to_id,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cities)
        .service(get_city_by_id)
        .service(upload_cities);
}

#[get("/v1/cities")]
async fn get_cities(
    city_service: Data<Arc<dyn CityService + Send + Sync>>,
) -> Result<web::Json<Vec<CityDto>>, Error> {
    // load cities
    let result = match city_service.into_inner().get_all() {
        Ok(cities) => cities,
        Err(err) => return Err(err),
    };
    // convert to DTOs
    let dtos: Vec<CityDto> = result.iter()
        .map(|c| CityDto::from_model(c))
        .collect();

    Ok(web::Json(dtos))
}

#[get("/v1/cities/{id}")]
async fn  get_city_by_id(
    id: web::Path<String>,
    city_service: Data<Arc<dyn CityService + Send + Sync>>,
) -> Result<web::Json<CityDto>, Error> {
    // get id
    let city_id = match string_to_id(id.to_string()) {
        Ok(v) => v,
        Err(err) => return Err(Error::bad_request(format!("failed to parse city ID: {}", err.to_string()))),
    };
    // load city
    let city = match city_service.into_inner().get_full(city_id) {
        Ok(city) => match city {
            Some(city) => CityDto::from_model(&city),
            None => return Err(Error::not_found("city not found".to_string())),
        },
        Err(err) => return Err(err),
    };

    Ok(web::Json(city))
}

#[post("/v1/cities")]
//#[roles("admin")]
async fn upload_cities(
    req: HttpRequest,
    payload: web::Bytes,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    city_service: Data<Arc<dyn CityService + Send + Sync>>,
) -> Result<impl Responder, Error> {
    get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    match city_service.into_inner().save_cities(payload.to_vec().as_slice()) {
        Ok(()) => Ok(HttpResponse::Created().body("saved all cities")),
        Err(err) => Err(err),
    }
}