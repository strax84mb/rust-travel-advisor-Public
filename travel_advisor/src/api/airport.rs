use std::sync::Arc;

use actix_web::{
    get,
    post,
    put,
    delete,
    web::{
        self,
        Data,
    },
    HttpRequest,
    HttpResponse,
    Responder,
};

use crate::{
    services::traits::{
        AirportService,
        AuthService,
    },
    util::Error,
};
use super::{
    dtos::AirportDto,
    validations::{
        extract_auth,
        string_to_id,
    },
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_airports)
        .service(get_airport_by_id)
        .service(create_airpot)
        .service(update_airpot)
        .service(delete_airpot)
        .service(upload_airpots);
}

#[get("/v1/airports")]
async fn get_airports(
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
) -> Result<web::Json<Vec<AirportDto>>, Error> {
    // load airports
    let result = match airport_service.into_inner().get_all() {
        Ok(loaded) => loaded,
        Err(err) => return Err(err),
    };
    // convert to DTO
    let result: Vec<AirportDto> = result.iter().map(|a| AirportDto::from_model(a)).collect();
    Ok(web::Json(result))
}

#[get("/v1/airports/{id}")]
async fn get_airport_by_id(
    id: web::Path<String>,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
) -> Result<web::Json<AirportDto>, Error> {
    // check param
    let id = match string_to_id(id.to_string()) {
        Ok(id) => id,
        Err(err) => return Err(Error::bad_request(format!("invalid ID: {}", err.to_string()))),
    };
    // load airport
    match airport_service.into_inner().get_by_id(id) {
        Ok(airport) => match airport {
            Some(airport) => Ok(web::Json(AirportDto::from_model(&airport))),
            None => Err(Error::not_found("airport not found".to_string())),
        },
        Err(err) => Err(err),
    }
}

#[post("/v1/airports")]
async fn create_airpot(
    req: HttpRequest,
    payload: web::Bytes,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> Result<web::Json<AirportDto>, Error> {
    // validate access right
    match auth_service.has_role(extract_auth(&req), vec!["admin"]) {
        Err(err) => return Err(err),
        Ok(has_rights) if !has_rights => return Err(Error::unauthorized_str("user has no rights for this operation")),
        _ => (),
    };
    // deserialize
    let dto: AirportDto = match serde_json::from_slice(payload.to_vec().as_slice()) {
        Ok(v) => v,
        Err(err) => return Err(Error::bad_request(format!("incorrect payload: {}", err.to_string()))),
    };
    let airport = dto.to_model();
    // save new airport
    match airport_service.into_inner().create(airport) {
        Ok(final_airport) => Ok(web::Json(AirportDto::from_model(&final_airport))),
        Err(err) => Err(err),
    }
}

#[put("/v1/airports/{id}")]
async fn update_airpot(
    req: HttpRequest,
    payload: web::Bytes,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> Result<impl Responder, Error> {
    // validate access right
    match auth_service.has_role(extract_auth(&req), vec!["admin"]) {
        Err(err) => return Err(err),
        Ok(has_rights) if !has_rights => return Err(Error::unauthorized_str("user has no rights for this operation")),
        _ => (),
    };
    // deserialize
    let dto: AirportDto = match serde_json::from_slice(payload.to_vec().as_slice()) {
        Ok(v) => v,
        Err(err) => return Err(Error::bad_request(format!("incorrect payload: {}", err.to_string()))),
    };
    let airport = dto.to_model();
    // save new airport
    match airport_service.into_inner().update(airport) {
        Ok(()) => Ok(HttpResponse::Created().finish()),
        Err(err) => Err(err),
    }
}

#[delete("/v1/airports/{id}")]
async fn delete_airpot(
    req: HttpRequest,
    id: web::Path<String>,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
) -> Result<impl Responder, Error> {
    match auth_service.has_role(extract_auth(&req), vec!["admin"]) {
        Err(err) => return Err(err),
        Ok(has_rights) if !has_rights => return Err(Error::unauthorized_str("user has no rights for this operation")),
        _ => (),
    };
    // check param
    let id = match string_to_id(id.to_string()) {
        Ok(id) => id,
        Err(err) => return Err(Error::bad_request(err.to_string())),
    };
    // delete airport
    match airport_service.into_inner().delete(id) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }
}

#[post("/v1/airports/upload")]
async fn upload_airpots(
    req: HttpRequest,
    payload: web::Bytes,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> Result<impl Responder, Error> {
    // validate access right
    match auth_service.has_role(extract_auth(&req), vec!["admin"]) {
        Err(err) => return Err(err),
        Ok(has_rights) if !has_rights => return Err(Error::unauthorized_str("user has no rights for this operation")),
        _ => (),
    };
    // save airports
    match airport_service.into_inner().save_airports(payload.to_vec().as_slice()) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err.wrap_str("failed to save all airports")),
    }
}
