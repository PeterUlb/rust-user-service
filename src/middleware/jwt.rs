use std::pin::Pin;
use std::task::{Context, Poll};

use crate::auth;
use crate::configuration;
use crate::error::ApiError;
use actix_service::{Service, Transform};
use actix_web::http::Method;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future;
use futures::Future;
use std::collections::HashMap;
use std::rc::Rc;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtAuth {
    jwt_config: configuration::Jwt,
    exempt_path: Rc<HashMap<String, Vec<Method>>>,
}

impl JwtAuth {
    pub fn new(
        jwt_config: configuration::Jwt,
        exempt_path: Rc<HashMap<String, Vec<Method>>>,
    ) -> Self {
        Self {
            jwt_config,
            exempt_path,
        }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for JwtAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(JwtAuthMiddleware {
            service: service,
            jwt_config: self.jwt_config.clone(),
            exempt_path: self.exempt_path.clone(),
        })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
    jwt_config: configuration::Jwt,
    exempt_path: Rc<HashMap<String, Vec<Method>>>,
}

impl<S, B> Service for JwtAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        debug!("Jwt Auth Middleware called for {}", req.path());

        let mut skip = match self.exempt_path.get(req.path()) {
            Some(v) => v.contains(req.method()),
            None => req.method() == Method::OPTIONS,
        };

        if !self.jwt_config.active {
            skip = true;
            // TODO: Add default claims
        }

        if skip == false {
            let token = match auth::get_auth_token(&req.headers()) {
                Some(token) => token,
                None => {
                    return Box::pin(async {
                        Err(ApiError::from(ApiError::NoAccessTokenHeader).into())
                    });
                }
            };

            let claims = match auth::decode_access_jwt(token, &self.jwt_config) {
                Ok(claims) => claims,
                Err(e) => {
                    error!("{}", e);
                    return Box::pin(async { Err(e.into()) });
                }
            };

            req.extensions_mut().insert(claims);
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
