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
    util::app_errors::{
        Error,
        Reason,
    }
};
use super::{
    dtos::CommentDto,
    validations::{
        extract_auth,
        string_to_id,
    },
    responses::{
        respond_bad_request,
        respond_ok,
        resolve_error,
        respond_created,
        respond_not_found,
        respond_forbidden,
    },
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_comments_for_user)
        .service(get_comments_for_city)
        .service(save_comment)
        .service(update_comment)
        .service(delete_comment);
}

#[get("/v1/users/{id}/comments")]
pub async fn get_comments_for_user(
    id: web::Path<String>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> impl Responder {
    // check param
    let id = match string_to_id(id.to_string()) {
        Ok(id) => id,
        Err(err) => return respond_bad_request(err.to_string()),
    };

    handle_comment_vec(comment_service.into_inner().list_for_user(id))
}

#[get("/v1/cities/{id}/comments")]
pub async fn get_comments_for_city(
    id: web::Path<String>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> impl Responder {
    // check param
    let id = match string_to_id(id.to_string()) {
        Ok(id) => id,
        Err(err) => return respond_bad_request(err.to_string()),
    };
    handle_comment_vec(comment_service.into_inner().list_for_city(id))
}

fn handle_comment_vec(promise: Result<Vec<Comment>, Error>) -> HttpResponse {
    let comments = match promise {
        Ok(comments) => comments,
        Err(err) => return resolve_error(err, Some("failed to load comments")),
    };
    let comments: Vec<CommentDto> = comments.iter().map(|c| CommentDto::from_model(c)).collect();
    respond_ok(Some(comments))
}

#[post("/v1/cities/{city_id}/comments")]
async fn save_comment(
    req: HttpRequest,
    city_id: web::Path<String>,
    payload: web::Json<CommentDto>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> impl Responder {
    // check path params
    let city_id = match string_to_id(city_id.to_string()) {
        Ok(id) => id,
        Err(err) => return respond_bad_request(err.to_string()),
    };
    // extract payload
    let mut comment = payload.0;
    comment.city_id = city_id;
    let mut comment = comment.to_model();
    // get user
    let user = match auth_service.into_inner().get_user(extract_auth(&req)) {
        Ok(user) => user,
        Err(err) => return resolve_error(err, Some("failed to load user")),
    };
    comment.user_id = user.id.clone();
    // save comment
    match comment_service.into_inner().create(user.id.clone(), comment) {
        Ok(comment) => respond_created(Some(comment)),
        Err(err) => resolve_error(err, None), 
    }
}

#[put("/v1/comments/{comment_id}")]
async fn update_comment(
    req: HttpRequest,
    comment_id: web::Path<String>, 
    payload: web::Json<CommentDto>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> impl Responder {
    let comment_service = comment_service.into_inner();
    // get user
    let user = match auth_service.into_inner().get_user(extract_auth(&req)) {
        Ok(user) => user,
        Err(err) => return resolve_error(err, Some("failed to load user")),
    };
    // extract path parameters
    let comment_id = match string_to_id(comment_id.to_string()) {
        Ok(id) => id,
        Err(err) => return respond_bad_request(err.to_string()),
    };
    // load comment
    let mut comment = match comment_service.get_by_id(comment_id) {
        Ok(comment) => match comment {
            Some(comment) => comment,
            None => return respond_not_found("comment not found"),
        },
        Err(err) => return resolve_error(err, None),
    };
    // extract payload
    comment.content = payload.0.content.clone();
    // update comment
    match comment_service.update(user.id.clone(), comment) {
        Ok(comment) => respond_created(Some(comment)),
        Err(err) if err.type_message(Reason::Forbidden).is_some() => resolve_error(err, Some("only poster can update comment")),
        Err(err) => resolve_error(err, None),
    }
}

#[delete("/v1/comments/{comment_id}")]
async fn delete_comment(
    req: HttpRequest,
    comment_id: web::Path<String>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    comment_service: Data<Arc<dyn CommentService + Send + Sync>>,
) -> impl Responder {
    // get user
    let user = match auth_service.into_inner().get_user(extract_auth(&req)) {
        Ok(user) => user,
        Err(err) => return resolve_error(err, Some("failed to load user")),
    };
    // extract path parameters
    let comment_id = match string_to_id(comment_id.to_string()) {
        Ok(id) => id,
        Err(_) => return respond_bad_request("invalid comment ID".to_string()),
    };
    // delete comment
    match comment_service.into_inner().delete(comment_id, user) {
        Ok(()) => respond_ok(None::<i64>),
        Err(err) if err.type_message(Reason::Forbidden).is_some() => respond_forbidden(None),
        Err(err) => resolve_error(err, None),
    }
}