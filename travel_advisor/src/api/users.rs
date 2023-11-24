use std::sync::Arc;

use actix_web::{
    post,
    web::{
        self,
        Data,
    },
    Responder,
};

use crate::{
    AuthService,
    UserRepository,
};
use super::{
    dtos::{
        LoginRequest,
        LoginResponse,
    }, 
    responses::{
        respond_not_found,
        resolve_error,
        respond_ok,
    }
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}

#[post("/v1/login")]
async fn login(
    payload: web::Json<LoginRequest>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    user_repo: Data<Arc<dyn UserRepository + Send + Sync>>,
) -> impl Responder {
    let request = payload.into_inner();
    let user = match user_repo.into_inner().get_by_email_and_pass(request.email, request.pass).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return respond_not_found("incorrect email or password"),
        },
        Err(err) => return resolve_error(err, Some("failed to load user")),
    };
    
    let user_data = match auth_service.create_jwt(user).await {
        Ok(data) => data,
        Err(err) => return resolve_error(err, Some("failed to generate JWT")),
    };

    let response = LoginResponse {
        id: user_data.user_id,
        token: user_data.jwt,
    };

    respond_ok(Some(response))
}
