use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
use crate::services::{get_reply, ClientMessage, Command};
use crate::users::get_user;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    u: String,
    p: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct A {
    t: String,
    r: i64,
}

#[derive(Serialize)]
struct C {
    r: bool,
}

pub async fn login(data: Auth) -> Result<A, ServiceError> {
    let reply = get_reply(&data.u, &data.p).ok_or(ServiceError::NotAuth)?;
    Ok(A {
        t: reply.0,
        r: reply.1,
    })
}

pub async fn check_auth(data: A) -> Result<C, ServiceError> {
    // dbg!(&data);
    let result = get_user(&data.t)
        .map(|u| u.role == data.r)
        .ok_or(ServiceError::NotAuth)?;
    Ok(C { r: result })
}

pub fn check(message: ClientMessage) -> Result<Command, ServiceError> {
    let user = get_user(&message.addon).ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
