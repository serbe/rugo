use actix_web::dev::ServiceRequest;
use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

use crate::db::{ClientMessage, Command};
use crate::error::ServiceError;

#[derive(Deserialize, Serialize)]
pub struct Auth {
    u: String,
    p: String,
}

#[derive(Deserialize, Serialize)]
struct A {
    t: String,
}

pub async fn login(data: web::Json<Auth>) -> Result<HttpResponse, ServiceError> {
    if &data.u == "user" && &data.p == "UserPass12" {
        Ok(HttpResponse::Ok().json(A {
            t: "dXNlclVzZXJQYXNzMTI=".to_string(),
        }))
    } else {
        Err(ServiceError::NotAuth)
    }
}

pub async fn validator(
    req: ServiceRequest,
    _credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    Ok(req)
}

pub fn check(message: ClientMessage) -> Result<Command, ServiceError> {
    if &message.addon != "dXNlclVzZXJQYXNzMTI=" {
        return Err(ServiceError::NotAuth);
    }
    Ok(message.command)
}
