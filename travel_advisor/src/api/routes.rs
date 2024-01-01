use std::sync::Arc;

use actix_web::{
    web::{
        self,
        Data,
    },
    HttpRequest,
    HttpResponse,
    Responder,
    get,
    delete,
    post,
    put,
};

use crate::{
    model::Route,
    services::traits::{
        AuthService,
        RouteService,
    },
    util::Error
};

use super::{
    get_user_if_has_roles,
    dtos::{
        FromModel,
        BestPathDto,
        CalculateCheapestRouteRequestDto,
        PaginationQueryParam,
        RouteDto,
        SaveRouteDto,
    },
    validations::get_number,
};

pub(super) fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/routes")
            .service(get_all)
            .service(find_by_id)
            .service(save_routes)
            .service(update_route)
            .service(delete_route)
            .service(find_cheapest_route)
    );
}

#[get("/{id}")]
async fn find_by_id(
    route_service: web::Data<Arc<dyn RouteService + Send + Sync>>,
    id: web::Path<String>,
) -> Result<web::Json<RouteDto>, Error> {
    let route_id = get_number!(id.to_string(), i64);
    match route_service.find_by_id(route_id) {
        Ok(route_opt) => match route_opt {
            Some(route) => Ok(web::Json(RouteDto::from_model(&route))),
            None => Err(Error::not_found("route not found".to_string())),
        },
        Err(err) => Err(err),
    }
}

#[get("")]
async fn get_all(
    route_service: web::Data<Arc<dyn RouteService + Send + Sync>>,
    query: web::Query<PaginationQueryParam>,
) -> Result<web::Json<Vec<RouteDto>>, Error> {
    let offset = match query.offset.clone() {
        Some(val) => get_number!(val, u64),
        None => 0,
    };
    let limit = match query.limit.clone() {
        Some(val) => get_number!(val, u64),
        None => 50,
    };
    match route_service.get_all(offset as i64, limit as i64) {
        Ok(routes) => Ok(web::Json(
            routes.iter().map(|r| RouteDto::from_model(r)).collect()
        )),
        Err(err) => Err(err),
    }
}

#[post("/uploads")]
async fn save_routes(
    req: HttpRequest,
    payload: web::Bytes,
    route_service: web::Data<Arc<dyn RouteService + Send + Sync>>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
) -> Result<impl Responder, Error> {
    get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    match route_service.save_routes(&payload.to_vec().as_slice()) {
        Ok(()) => Ok(HttpResponse::Created().finish()),
        Err(err) => Err(err),
    }
}

#[put("/{id}")]
async fn update_route(
    req: HttpRequest,
    id: web::Path<String>,
    body: web::Json<SaveRouteDto>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    route_service: web::Data<Arc<dyn RouteService + Send + Sync>>,
) -> Result<impl Responder, Error> {
    get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    let route_id = get_number!(id.to_string(), i64);
    let route = Route {
        id: route_id,
        start: body.start.clone(),
        finish: body.finish.clone(),
        price: body.price.clone(),
    };
    match route_service.update(route) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }
}

#[delete("/{id}")]
async fn delete_route(
    req: HttpRequest,
    id: web::Path<String>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    route_service: web::Data<Arc<dyn RouteService + Send + Sync>>,
) -> Result<impl Responder, Error> {
    get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    let route_id = get_number!(id.to_string(), i64);
    match route_service.delete(route_id) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }
}

#[post("/cheapest-path")]
async fn find_cheapest_route(
    req: HttpRequest,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    body: web::Json<CalculateCheapestRouteRequestDto>,
    route_service: web::Data<Arc<dyn RouteService + Send + Sync>>,
) -> Result<web::Json<BestPathDto>, Error> {
    get_user_if_has_roles!(req, auth_service, vec!["admin"]);
    let (routes, airports, cities) = match route_service.find_cheapest_route(
        body.starting_city_id.clone(),
        body.destination_city_id.clone()
    ) {
        Ok((r, a, c)) => (r, a, c),
        Err(err) => return Err(err),
    };
    match BestPathDto::from_models(routes, airports, cities) {
        Ok(path) => Ok(web::Json(path)),
        Err(err) => Err(err),
    }
}