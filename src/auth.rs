use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db::{get_reply, get_user, ClientMessage, Command};
use crate::error::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    u: String,
    p: String,
}

#[derive(Deserialize, Serialize)]
struct A {
    t: String,
    r: i64,
}

pub async fn login(data: web::Json<Auth>) -> Result<HttpResponse, ServiceError> {
    let reply = get_reply(&data.u, &data.p).ok_or(ServiceError::NotAuth)?;
    Ok(HttpResponse::Ok().json(A {
        t: reply.0,
        r: reply.1,
    }))
}

pub fn check(message: ClientMessage) -> Result<Command, ServiceError> {
    let user = get_user(&message.addon).ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
