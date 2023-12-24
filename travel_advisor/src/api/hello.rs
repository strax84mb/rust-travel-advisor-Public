use std::sync::Arc;

use actix_web::{
    get,
    post,
    HttpResponse,
    Responder,
    web,
};
use serde::Serialize;

use crate::{
    services::traits::CityService,
    util::Error,
};
use super::dtos::CityDto;
 
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/hello")
            .service(hello_world)
            .service(
                web::resource("/test-traits")
                    .guard(actix_web::guard::Get())
                    .to(take_test_for_a_ride)
            ).service(test_save_city)
            .service(hello_with_json_payloads)
    );
}

#[get("")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("World!")
}

async fn take_test_for_a_ride(
    city_service: web::Data<Arc<dyn CityService + Send + Sync>>,
) -> Result<web::Json<Vec<CityDto>>, Error> {
    match city_service.into_inner().get_all() {
        Ok(result) => {
            println!(">>> count: {}", result.len());
            let dtos: Vec<CityDto> = result.iter().map(|c| CityDto::from_model(c)).collect();
            Ok(web::Json(dtos))
        },
        Err(err) => {
            println!(">>> error: {}", err.to_string());
            Err(err)
        },
    }
}

#[post("/city/{name}")]
async fn test_save_city(
    name: web::Path<String>,
    city_service: web::Data<Arc<dyn CityService + Send + Sync>>,
) -> Result<web::Json<CityDto>, Error> {
    match city_service.new(name.to_string()) {
        Ok(city) => Ok(web::Json(CityDto::from_model(&city))),
        Err(err) => Err(err),
    }
}

#[derive(Serialize)]
struct HelloTestPayload {
    id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    active: bool,
}

#[derive(Serialize, Debug, derive_more::Display, derive_more::Error)]
enum TestErrorEnum {
    #[display(fmt="internal")]
    Internal(TestError),

    #[display(fmt="internal")]
    NotFound(TestError),
}

#[derive(Serialize, Debug, derive_more::Error, derive_more::Display)]
#[display(fmt = "error")]
struct TestError {
    code: String,
    description: String,
}

impl actix_web::ResponseError for TestErrorEnum {

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let (mut builder, payload) = match self {
            Self::Internal(p) => (HttpResponse::InternalServerError(), p),
            Self::NotFound(p) => (HttpResponse::NotFound(), p),
        };
        match serde_json::to_string(payload) {
            Ok(str_payload) => builder.body(str_payload),
            Err(err) => HttpResponse::InternalServerError().body(
                format!("failed to serialize payload. Cause: {}", err.to_string())
            )
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::Internal(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
        }
    }

}

#[get("/test-payloads/{type}")]
async fn hello_with_json_payloads(
    payload_type: web::Path<String>,
) -> Result<web::Json<HelloTestPayload>, TestErrorEnum> {
    match payload_type.as_str() {
        "test" => Ok(web::Json(HelloTestPayload {
            id: 1,
            name: None,
            active: false,
        })),
        "named" => Ok(web::Json(HelloTestPayload {
            id: 2,
            name: Some("Strale".to_string()),
            active: true,
        })),
        "not-found" => Err(TestErrorEnum::NotFound(TestError {
            code: "NOT_FOUND".to_string(),
            description: "Did not find what I need!!!".to_string(),
        })),
        _ => Err(TestErrorEnum::Internal(TestError {
                code: "TEST_CODE".to_string(),
                description: "Some text to see".to_string(),
            }
        )),
    }
}