use std::sync::Arc;
use std::collections::HashSet;
use std::{env, future::{ready, Ready}};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Validation, DecodingKey};
use actix_web::{Error, body::EitherBody, dev::{self, Service, ServiceRequest, ServiceResponse, Transform}};

use crate::{helpers::response::response_json, structs::auth_struct::TokenStruct};

pub struct CheckCookie;

impl<S, B> Transform<S, ServiceRequest> for CheckCookie
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type InitError = ();
  type Transform = CheckCookieMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(CheckCookieMiddleware { service }))
  }
}
pub struct CheckCookieMiddleware<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for CheckCookieMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  dev::forward_ready!(service);

  fn call(&self, request: ServiceRequest) -> Self::Future {
    let path = request.path().to_string();
    let method = request.method().to_string();

    if path == "/" || path == "/login" || path == "/register" || path.starts_with("/join") || path.starts_with("/application") {
      let res = self.service.call(request);

      return Box::pin(async move {
        res.await.map(ServiceResponse::map_into_left_body)
      });
    }

    let jwt_title = env::var("JWT_TOKEN_TITLE").unwrap_or_else(|_| String::from("auth_jwt_secret"));
    let jwt_secret = env::var("JWT_TOKEN_SECRET").unwrap_or_else(|_| String::from("secret"));
    let token = request.cookie(&jwt_title);
    let validation = Validation::default();

    match token {
      Some(token) => {
        match decode::<TokenStruct> (
          &token.value(),
          &DecodingKey::from_secret(jwt_secret.as_ref()),
          &validation
        ) {
          Ok(data_token) => {
            let whitelist_routes: HashSet<String> = vec![
              "/forum".to_owned(),
              "/student".to_owned(),
              "/university".to_owned(),
              "/schoolarship".to_owned(),
            ].into_iter().collect();

            let whitelist_routes = Arc::new(whitelist_routes);

            if whitelist_routes.contains(&path) && data_token.claims.role == "user" {
              if method == "POST" || method == "PUT" || method == "DELETE" {
                let request = request.into_parts().0;

                let response = response_json(
                  "unauthorize".to_string(),
                  "you are not allowed to access this route".to_string(),
                  vec![]
                ).map_into_right_body();

                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
              } else {
                let res = self.service.call(request);

                return Box::pin(async move {
                  res.await.map(ServiceResponse::map_into_left_body)
                });
              }
            } else {
              let res = self.service.call(request);

              return Box::pin(async move {
                res.await.map(ServiceResponse::map_into_left_body)
              });
            }
          },
          Err(_) => {
            let request = request.into_parts().0;

            let response = response_json(
              "unauthorize".to_string(),
              "something went wrong from your cookies".to_string(),
              vec![]
            ).map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
          }
        }
      }
      None => {
        let request = request.into_parts().0;

        let response = response_json(
          "unauthorize".to_string(),
          "please authorize your self as user".to_string(),
          vec![]
        ).map_into_right_body();

        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
      }
    }
  }
}