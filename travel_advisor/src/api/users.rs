use std::sync::Arc;

use actix_web::{
    post,
    web::{
        self,
        Data,
    },
};

use crate::{
    AuthService,
    UserRepository,
    util::Error,
};
use super::dtos::{
    LoginRequest,
    LoginResponse,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}

#[post("/v1/login")]
async fn login(
    payload: web::Json<LoginRequest>,
    auth_service: Data<Arc<dyn AuthService + Send + Sync>>,
    user_repo: Data<Arc<dyn UserRepository + Send + Sync>>,
) -> Result<web::Json<LoginResponse>, Error> {
    let request = payload.into_inner();
    let user = match user_repo.get_by_email_and_pass(request.email, request.pass) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(Error::not_found("incorrect email or password".to_string())),
        },
        Err(err) => return Err(err.wrap_str("failed to load user")),
    };
    
    let user_data = match auth_service.create_jwt(user) {
        Ok(data) => data,
        Err(err) => return Err(err.wrap_str("failed to generate JWT")),
    };

    let response = LoginResponse {
        id: user_data.user_id,
        token: user_data.jwt,
    };

    Ok(web::Json(response))
}
