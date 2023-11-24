use actix_web::{
    HttpResponse,
    HttpResponseBuilder,
    body::BoxBody,
};
use serde::Serialize;

use crate::util::app_errors::{
    Error,
    Reason,
};

pub enum ErrorType {
    BadRequest,
    NotFound,
    Forbidden,
    Base,
}

impl ErrorType {
    fn to_string(&self) -> String {
        match self {
            Self::BadRequest => "BAD_REQUEST".to_string(),
            Self::NotFound => "NOT_FOUND".to_string(),
            Self::Forbidden => "FORBIDDEN".to_string(),
            Self::Base => "BASE_ERROR".to_string(),
        }
    }
}

#[derive(Serialize)]
struct ErrorPayload {
    code: String,
    message: String,
}

impl ErrorPayload {
    fn new(error_type: ErrorType, message: String) -> ErrorPayload {
        ErrorPayload {
            code: error_type.to_string(),
            message: message,
        }
    }
}

fn serialize_payload(mut builder: HttpResponseBuilder, payload_type: &str, obj: impl Serialize) -> HttpResponse<BoxBody> {
    match serde_json::to_string(&obj) {
        Ok(val) => builder.body(val),
        Err(err) => HttpResponse::InternalServerError().body(format!("failed to serialize {}: {}", payload_type, err.to_string())),
    }
}

pub fn resolve_error(err: Error, msg: Option<&str>) -> HttpResponse<BoxBody> {
    let mut text = err.type_message(Reason::Forbidden);
    if text.is_some() {
        return serialize_payload(HttpResponse::Forbidden(), "error", ErrorPayload::new(ErrorType::Forbidden, text.unwrap()));
    }
    text = err.type_message(Reason::NotFound);
    if text.is_some() {
        return serialize_payload(HttpResponse::NotFound(), "error", ErrorPayload::new(ErrorType::NotFound, text.unwrap()));
    }
    let message = match msg {
        Some(msg) => format!("{}: {}", msg, err.to_string()),
        None => err.to_string(),
    };
    serialize_payload(
        HttpResponse::InternalServerError(),
        "error",
        ErrorPayload::new(ErrorType::Base, message),
    )
}

pub fn respond_not_found(msg: &str) -> HttpResponse<BoxBody> {
    serialize_payload(
        HttpResponse::NotFound(),
        "error",
        ErrorPayload::new(ErrorType::NotFound, msg.to_string()),
    )
}

pub fn respond_bad_request(msg: String) -> HttpResponse<BoxBody> {
    serialize_payload(
        HttpResponse::BadRequest(),
        "error",
        ErrorPayload::new(ErrorType::BadRequest, msg),
    )
}

pub fn respond_unauthorized(msg: Option<String>) -> HttpResponse<BoxBody> {
    match msg {
        Some(msg) => HttpResponse::Unauthorized().body(msg),
        None => HttpResponse::Unauthorized().finish(),
    }
}

pub fn respond_forbidden(msg: Option<&str>) -> HttpResponse<BoxBody> {
    match msg {
        Some(msg) => HttpResponse::Forbidden().body(msg.to_string()),
        None => HttpResponse::Forbidden().finish(),
    }
}

pub fn respond_ok(payload: Option<impl Serialize>) -> HttpResponse<BoxBody> {
    match payload {
        Some(payload) => serialize_payload(HttpResponse::Ok(), "payload", payload),
        None => HttpResponse::Ok().finish(),
    }
}

pub fn respond_created(payload: Option<impl Serialize>) -> HttpResponse<BoxBody> {
    match payload {
        Some(payload) => serialize_payload(HttpResponse::Created(), "payload", payload),
        None => HttpResponse::Created().finish(),
    }
}
