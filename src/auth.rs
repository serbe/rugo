use actix_web::{web, HttpResponse};
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
    let key = get_key(&data.u, &data.p).ok_or(ServiceError::NotAuth)?;
    Ok(HttpResponse::Ok().json(A { t: key }))
}

pub fn check(message: ClientMessage) -> Result<Command, ServiceError> {
    let user = get_user(&message.addon).ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
