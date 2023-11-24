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
    Responder,
};

use crate::services::traits::{
    AirportService,
    AuthService,
};
use super::{
    dtos::AirportDto,
    validations::{
        extract_auth,
        string_to_id,
    },
    responses::{
        resolve_error,
        respond_ok,
        respond_created,
        respond_not_found,
        respond_bad_request,
        respond_unauthorized,
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
async fn get_airports(airport_service: Data<Arc<dyn AirportService + Send + Sync>>) -> impl Responder {
    // load airports
    let result = match airport_service.into_inner().get_all().await {
        Ok(loaded) => loaded,
        Err(err) => return resolve_error(err, None),
    };
    // convert to DTO
    let result: Vec<AirportDto> = result.iter().map(|a| AirportDto::from_model(a)).collect();
    respond_ok(Some(result))
}

#[get("/v1/airports/{id}")]
async fn get_airport_by_id(
    id: web::Path<String>,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
) -> impl Responder {
    // check param
    let id = match string_to_id(id.to_string()) {
        Ok(id) => id,
        Err(err) => return respond_bad_request(format!("invalid ID: {}", err.to_string())),
    };
    // load airport
    let result = match airport_service.into_inner().get_by_id(id).await {
        Ok(airport) => match airport {
            Some(airport) => AirportDto::from_model(&airport),
            None => return respond_not_found("airport not found"),
        },
        Err(err) => return resolve_error(err, None),
    };
    // serialize
    respond_ok(Some(result))
}

#[post("/v1/airports")]
async fn create_airpot(
    req: HttpRequest,
    payload: web::Bytes,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> impl Responder {
    // validate access right
    match auth_service.has_role(extract_auth(&req), vec!["admin"]).await {
        Err(err) => return respond_unauthorized(Some(err.to_string())),
        _ => (),
    };
    // deserialize
    let dto: AirportDto = match serde_json::from_slice(payload.to_vec().as_slice()) {
        Ok(v) => v,
        Err(err) => return respond_bad_request(format!("incorrect payload: {}", err.to_string())),
    };
    let airport = dto.to_model();
    // save new airport
    match airport_service.into_inner().create(airport).await {
        Ok(final_airport) => respond_ok(Some(final_airport)),
        Err(err) => resolve_error(err, None),
    }
}

#[put("/v1/airports/{id}")]
async fn update_airpot(
    req: HttpRequest,
    payload: web::Bytes,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> impl Responder {
    // validate access right
    match auth_service.has_role(extract_auth(&req), vec!["admin"]).await {
        Err(err) => return respond_unauthorized(Some(err.to_string())),
        _ => (),
    };
    // deserialize
    let dto: AirportDto = match serde_json::from_slice(payload.to_vec().as_slice()) {
        Ok(v) => v,
        Err(err) => return respond_bad_request(format!("incorrect payload: {}", err.to_string())),
    };
    let airport = dto.to_model();
    // save new airport
    match airport_service.into_inner().update(airport).await {
        Ok(()) => respond_created(None::<i8>),
        Err(err) => resolve_error(err, None),
    }
}

#[delete("/v1/airports/{id}")]
async fn delete_airpot(
    req: HttpRequest,
    id: web::Path<String>,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> impl Responder {
    match auth_service.has_role(extract_auth(&req), vec!["admin"]).await {
        Err(err) => return respond_unauthorized(Some(err.to_string())),
        _ => (),
    };
    // check param
    let id = match string_to_id(id.to_string()) {
        Ok(id) => id,
        Err(err) => return respond_bad_request(err.to_string()),
    };
    // delete airport
    match airport_service.into_inner().delete(id).await {
        Ok(()) => respond_ok(None::<i8>),
        Err(err) => resolve_error(err, None),
    }
}

#[post("/v1/airports/upload")]
async fn upload_airpots(
    req: HttpRequest,
    payload: web::Bytes,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> impl Responder {
    // validate access right
    match auth_service.has_role(extract_auth(&req), vec!["admin"]).await {
        Err(err) => return respond_unauthorized(Some(err.to_string())),
        _ => (),
    };
    // save airports
    match airport_service.into_inner().save_airports(payload.to_vec().as_slice()).await {
        Ok(()) => respond_ok(None::<i8>),
        Err(err) => resolve_error(err, Some("failed to save all airports")),
    }
}
