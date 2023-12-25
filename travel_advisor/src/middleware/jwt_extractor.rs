use std::{
    future::{
        ready,
        Ready,
    },
    cell::RefCell,
    sync::Arc,
};

use actix_web::{
    dev::{
        Transform,
        Service,
        ServiceRequest,
        ServiceResponse,
    },
    Error,
    HttpResponse,
    body::EitherBody,
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use crate::services::traits::AuthService;

pub struct JwtExtractor {
    auth_service: Arc<dyn AuthService + Send + Sync>,
}

pub fn new_jwt_extractor(auth_service: Arc<dyn AuthService + Send + Sync>) -> JwtExtractor {
    JwtExtractor {
        auth_service: auth_service,
    }
}


impl <S, B> Transform<S, ServiceRequest> for JwtExtractor
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{

    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtExtractorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtExtractorMiddleware {
            service: service,
            auth_service: self.auth_service.clone(),
        }))
    }

}

pub struct JwtExtractorMiddleware<S> {
    service: S,
    auth_service: Arc<dyn AuthService + Send + Sync>,
}

impl<S, B> Service<ServiceRequest> for JwtExtractorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
    
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt = match req.headers().get(actix_web::http::header::AUTHORIZATION) {
            Some(hv) => Some(hv.to_str()),
            None => None,
        };
        if jwt.is_some() {
            let user = match self.auth_service.get_user(jwt) {
                Ok(user) => user,
                Err(err) => match err {
                    crate::util::Error::NotFound(e) => return Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::Unauthorized().body(e.to_string()).map_into_right_body()
                        ))
                    }),
                    crate::util::Error::Unauthorized(e) => return Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::Unauthorized().body(e.to_string()).map_into_right_body()
                        ))
                    }),
                    crate::util::Error::Internal(e) => return Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::InternalServerError().body(e.to_string()).map_into_right_body()
                        ))
                    }),
                    _ => return Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::InternalServerError().body("UNFORSEEN_ERROR").map_into_right_body()
                        ))
                    }),
                },
            };
            let jwt_extension = crate::util::JwtExtension {
                user_id: user.id.clone(),
                user_name: user.email.clone(),
                roles: user.roles.clone(),
            };
            req.extensions_mut().insert(jwt_extension);
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body()) 
        })
    }
}
