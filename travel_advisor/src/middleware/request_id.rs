
use std::future::{
    ready,
    Ready,
};

use actix_web::{
    dev::{
        Transform,
        ServiceRequest,
        ServiceResponse,
        Service,
    },
    Error,
    http::header::{
        HeaderName,
        HeaderValue,
    },
};
use futures_util::future::LocalBoxFuture;

pub struct RequestId;

impl <S> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{

    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddleware { service }))
    }

}

pub struct RequestIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
    
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let found = req.headers().contains_key("request-id");
        if !found {
            let id = uuid::Uuid::new_v4().to_string();
            let id_header = HeaderValue::from_str(id.as_str()).unwrap();
            let headers = req.headers_mut();
            headers.append(HeaderName::from_static("request-id"), id_header);
        };
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res) 
        })
    }
}
