use actix_web::{
    http::StatusCode,
    HttpResponse,
    ResponseError,
};
use derive_more::{
    Display,
    Error,
};
use serde::Serialize;

#[derive(Serialize, Debug, Display, Error)]
pub enum ErrorV2 {
    #[display(ft="internal")]
    Internal(ErrorV2Payload),

    #[display(fmt="not found")]
    NotFound(ErrorV2Payload),

    #[display(fmt="forbidden")]
    Forbidden(ErrorV2Payload),

    #[display(fmt="bad request")]
    BadRequest(ErrorV2Payload),

    #[display(fmt="unauthorized")]
    Unauthorized(ErrorV2Payload)
}

#[derive(Serialize, Debug, Clone, Display, Error)]
#[display(fmt="error")]
pub struct ErrorV2Payload {
    code: ErrorCode,
    description: String,
}

fn do_wrap(msg: String, p: ErrorV2Payload) -> ErrorV2Payload {
    ErrorV2Payload {
        code: p.code,
        description: format!("{}: {}", msg, p.description),
    }
}

impl ErrorV2 {

    pub fn internal(code: ErrorCode, msg: String) -> Self {
        Self::Internal(ErrorV2Payload{
            code: code,
            description: msg,
        })
    }

    pub fn internal_str(code: ErrorCode, msg: &str) -> Self {
        Self::Internal(ErrorV2Payload{
            code: code,
            description: msg.to_string(),
        })
    }

    pub fn not_found(msg: String) -> Self {
        Self::NotFound(ErrorV2Payload{
            code: ErrorCode::EntityNotFound,
            description: msg,
        })
    }

    pub fn forbidden(msg: String) -> Self {
        Self::Forbidden(ErrorV2Payload {
            code: ErrorCode::ForbiddenResource,
            description: msg,
        })
    }

    pub fn forbidden_str(msg: &str) -> Self {
        Self::forbidden(msg.to_string())
    }

    pub fn unauthorized(msg: String) -> Self {
        Self::Unauthorized(ErrorV2Payload {
            code: ErrorCode::Unauthorized,
            description: msg,
        })
    }

    pub fn unauthorized_str(msg: &str) -> Self {
        Self::Unauthorized(ErrorV2Payload {
            code: ErrorCode::Unauthorized,
            description: msg.to_string(),
        })
    }

    pub fn bad_request(msg: String) -> Self {
        Self::BadRequest(ErrorV2Payload {
            code: ErrorCode::ValidationError,
            description: msg,
        })
    }

    pub fn wrap(&self, msg: String) -> Self {
        match self {
            Self::Internal(p) => Self::Internal(do_wrap(msg, p.clone())),
            Self::NotFound(p) => Self::NotFound(do_wrap(msg, p.clone())),
            Self::Forbidden(p) => Self::Forbidden(do_wrap(msg, p.clone())),
            Self::BadRequest(p) => Self::BadRequest(do_wrap(msg, p.clone())),
            Self::Unauthorized(p) => Self::Unauthorized(do_wrap(msg, p.clone())),
        }
    }

    pub fn wrap_str(&self, msg: &str) -> Self {
        match self {
            Self::Internal(p) => Self::Internal(do_wrap(msg.to_string(), p.clone())),
            Self::NotFound(p) => Self::NotFound(do_wrap(msg.to_string(), p.clone())),
            Self::Forbidden(p) => Self::Forbidden(do_wrap(msg.to_string(), p.clone())),
            Self::BadRequest(p) => Self::BadRequest(do_wrap(msg.to_string(), p.clone())),
            Self::Unauthorized(p) => Self::Unauthorized(do_wrap(msg.to_string(), p.clone())),
        }
    }

}

impl ResponseError for ErrorV2 {

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let (mut builder, payload) = match self {
            Self::Internal(p) => (HttpResponse::InternalServerError(), p),
            Self::NotFound(p) => (HttpResponse::NotFound(), p),
            Self::Forbidden(p) => (HttpResponse::Forbidden(), p),
            Self::BadRequest(p) => (HttpResponse::BadRequest(), p),
            Self::Unauthorized(p) => (HttpResponse::Unauthorized(), p),
        };
        let payload_str = match serde_json::to_string(&payload) {
            Ok(p) => p,
            Err(_err) => 
                return HttpResponse::InternalServerError().body(
                    format!("{{\"code\":\"{}\",\"description\":\"failed to serialize response payload\"}}", ErrorCode::SerializeError.to_string())
                ),
        };
        builder.body(payload_str)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }

}

#[derive(Debug, Clone, Serialize, Display)]
pub enum ErrorCode {

    #[display(fmt="INTERNAL_ERROR")]
    InternalError,

    #[display(fmt="ENTITY_NOT_FOUND")]
    EntityNotFound,

    #[display(fmt="DB_READ_ERROR")]
    DbRead,

    #[display(fmt="DB_SAVE_ERROR")]
    DbSave,

    #[display(fmt="DB_DELETE_ERROR")]
    DbDelete,

    #[display(fmt="GET_DB_CONNECTIONS")]
    GetDbConnection,

    #[display(fmt="SERIALIZATION_ERROR")]
    SerializeError,

    #[display(fmt="FORBIDDEN_RESOURCE")]
    ForbiddenResource,

    #[display(fmt="UNAUTHORIZED")]
    Unauthorized,

    #[display(fmt="NO_AUTHORIZATION_HEADER")]
    NoAuthorizationHeader,

    #[display(fmt="JWT_EXPIRED")]
    JwtExpired,

    #[display(fmt="JWT_NOT_ACTIVE")]
    JwtNotActive,

    #[display(fmt="JWT_MALFORMED")]
    JwtMalformed,

    #[display(fmt="TEXT_ROW_PARSE")]
    TextRowParse,

    #[display(fmt="VALIDATION_ERROR")]
    ValidationError,

    #[display(fmt="SERIALIZATION_ERROR")]
    SerializationError,
}
