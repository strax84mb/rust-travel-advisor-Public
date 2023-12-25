use std::sync::Arc;

use actix_web::{
    delete,
    get,
    post,
    put,
    web::{
        self,
        Data,
    },
    Responder,
    HttpRequest,
    HttpResponse,
};

use crate::{
    AuthService,
    CommentService,
    model::Comment,
    util::{
        Error,
        ErrorCode,
    },
};
use super::{
    get_user_if_has_roles,
    dtos::CommentDto,
    validations::string_to_id,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(get_comments_for_user)
            .service(
                web::scope("/cities")
                    .service(get_comments_for_city)
                    .service(save_comment)
            ).service(
                web::scope("/comments")
                    .service(update_comment)
                    .service(delete_comment)
            )
    );
}

#[get("/users/{id}/comments")]
pub async fn get_comments_for_user(
    id: web::Path<String>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> Result<web::Json<Vec<CommentDto>>, Error> {
    // check param
    let id = match string_to_id(id.to_string()) {
        Ok(id) => id,
        Err(err) => return Err(Error::bad_request(err.to_string())),
    };

    handle_comment_vec(comment_service.into_inner().list_for_user(id))
}

#[get("/{id}/comments")]
pub async fn get_comments_for_city(
    id: web::Path<String>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> Result<web::Json<Vec<CommentDto>>, Error> {
    // check param
    let id = match string_to_id(id.to_string()) {
        Ok(id) => id,
        Err(err) => return Err(Error::bad_request(err.to_string())),
    };
    handle_comment_vec(comment_service.into_inner().list_for_city(id))
}

fn handle_comment_vec(promise: Result<Vec<Comment>, Error>) -> Result<web::Json<Vec<CommentDto>>, Error> {
    let comments = match promise {
        Ok(comments) => comments,
        Err(err) => return Err(err.wrap_str("failed to load comments")),
    };
    let comments: Vec<CommentDto> = comments.iter().map(|c| CommentDto::from_model(c)).collect();
    Ok(web::Json(comments))
}

#[post("/{city_id}/comments")]
async fn save_comment(
    req: HttpRequest,
    city_id: web::Path<String>,
    payload: web::Json<CommentDto>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> Result<web::Json<CommentDto>, Error> {
    let user = get_user_if_has_roles!(req, auth_service, vec!["admin", "user"]);
    // check path params
    let city_id = match string_to_id(city_id.to_string()) {
        Ok(id) => id,
        Err(err) => return Err(Error::bad_request(err.to_string())),
    };
    // extract payload
    let mut comment = payload.0;
    comment.city_id = city_id;
    let mut comment = comment.to_model();
    comment.user_id = user.id.clone();
    // save comment
    comment = match comment_service.into_inner().create(user.id.clone(), comment) {
        Ok(comment) => comment,
        Err(err) => return Err(err), 
    };
    let mut dto = CommentDto::from_model(&comment);
    dto.user_name = Some(user.email.clone());
    Ok(web::Json(dto))
}

#[put("/{comment_id}")]
async fn update_comment(
    req: HttpRequest,
    comment_id: web::Path<String>, 
    payload: web::Json<CommentDto>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> Result<impl Responder, Error> {
    let user = get_user_if_has_roles!(req, auth_service, vec!["admin", "user"]);
    let comment_service = comment_service.into_inner();
    // extract path parameters
    let comment_id = match string_to_id(comment_id.to_string()) {
        Ok(id) => id,
        Err(err) => return Err(Error::bad_request(err.to_string())),
    };
    // load comment
    let mut comment = match comment_service.get_by_id(comment_id) {
        Ok(comment) => match comment {
            Some(comment) => comment,
            None => return Err(Error::not_found("comment not found".to_string())),
        },
        Err(err) => return Err(err),
    };
    // extract payload
    comment.content = payload.0.content.clone();
    // update comment
    match comment_service.update(user.id.clone(), comment) {
        Ok(comment) => {
            let mut dto = CommentDto::from_model(&comment);
            dto.user_name = Some(user.email.clone());
            match serde_json::to_string(&dto) {
                Ok(json) => Ok(HttpResponse::Created().body(json)),
                Err(_err) => Err(Error::internal_str(ErrorCode::SerializeError, "failed to serialize response to json")),
            }
        },
        Err(err) => match err {
            Error::Forbidden(_) => Err(Error::forbidden_str("only poster can update comment")),
            _ => Err(err),
        }
    }
}

#[delete("/{comment_id}")]
async fn delete_comment(
    req: HttpRequest,
    comment_id: web::Path<String>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> Result<impl Responder, Error> {
    let user = get_user_if_has_roles!(req, auth_service, vec!["admin", "user"]);
    // extract path parameters
    let comment_id = match string_to_id(comment_id.to_string()) {
        Ok(id) => id,
        Err(_) => return Err(Error::bad_request("invalid comment ID".to_string())),
    };
    // delete comment
    match comment_service.into_inner().delete(comment_id, user) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err),
    }
}