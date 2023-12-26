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
    get_user_if_has_roles,
    dtos::{
        FromModel,
        ToModel,
        AirportDto,
        CreateAirportDto,
    },
    validations::get_number,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/airports")
        .service(get_airports)
        .service(get_airport_by_id)
        .service(create_airpot)
        .service(update_airpot)
        .service(delete_airpot)
        .service(upload_airpots)
    );
}

#[get("")]
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

#[get("/{id}")]
async fn get_airport_by_id(
    id: web::Path<String>,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
) -> Result<web::Json<AirportDto>, Error> {
    // check param
    let id = get_number!(id, i64, true);
    // load airport
    match airport_service.into_inner().get_by_id(id) {
        Ok(airport) => match airport {
            Some(airport) => Ok(web::Json(AirportDto::from_model(&airport))),
            None => Err(Error::not_found("airport not found".to_string())),
        },
        Err(err) => Err(err),
    }
}

#[post("")]
async fn create_airpot(
    req: HttpRequest,
    payload: web::Bytes,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> Result<web::Json<AirportDto>, Error> {
    // validate access right
    get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    // deserialize
    let dto: CreateAirportDto = match serde_json::from_slice(payload.to_vec().as_slice()) {
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

#[put("/{id}")]
async fn update_airpot(
    req: HttpRequest,
    payload: web::Bytes,
    id: web::Path<String>,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> Result<impl Responder, Error> {
    // validate access right
    get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    // get id
    let airport_id = get_number!(id, i64);
    // deserialize
    let dto: CreateAirportDto = match serde_json::from_slice(payload.to_vec().as_slice()) {
        Ok(v) => v,
        Err(err) => return Err(Error::bad_request(format!("incorrect payload: {}", err.to_string()))),
    };
    let airport = {
        let mut airport_mut = dto.to_model();
        airport_mut.id = airport_id;
        airport_mut
    };
    // save new airport
    match airport_service.into_inner().update(airport) {
        Ok(()) => Ok(HttpResponse::Created().finish()),
        Err(err) => Err(err),
    }
}

#[delete("/{id}")]
async fn delete_airpot(
    req: HttpRequest,
    id: web::Path<String>,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
) -> Result<impl Responder, Error> {
    get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    // check param
    let id = get_number!(id, i64, true);
    // delete airport
    match airport_service.into_inner().delete(id) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }
}

#[post("/upload")]
async fn upload_airpots(
    req: HttpRequest,
    payload: web::Bytes,
    airport_service: Data<Arc<dyn AirportService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>
) -> Result<impl Responder, Error> {
    // validate access right
    get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    // save airports
    match airport_service.into_inner().save_airports(payload.to_vec().as_slice()) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err.wrap_str("failed to save all airports")),
    }
}
