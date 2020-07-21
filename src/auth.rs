// use actix_web::dev::ServiceRequest;
use actix_web::{web, HttpResponse};
// use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

use crate::db::{get_key, get_user, ClientMessage, Command};
use crate::error::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    u: String,
    p: String,
}

#[derive(Deserialize, Serialize)]
struct A {
    t: String,
}

pub async fn login(data: web::Json<Auth>) -> Result<HttpResponse, ServiceError> {
    println!("{:?}", data);
    println!("{:?}", get_key(&data.u, &data.p));
    let key = get_key(&data.u, &data.p).ok_or(ServiceError::NotAuth)?;
    Ok(HttpResponse::Ok().json(A { t: key }))
}

// pub async fn validator(
//     req: ServiceRequest,
//     _credentials: BearerAuth,
// ) -> Result<ServiceRequest, Error> {
//     println!("{}", req.path());
//     Ok(req)
// }

pub fn check(message: ClientMessage) -> Result<Command, ServiceError> {
    let user = get_user(&message.addon).ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
