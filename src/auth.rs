use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
use crate::services::{ClientMessage, Command};
use crate::users::Users;

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

pub async fn login(users: &Users, data: Auth) -> Result<String, ServiceError> {
    let reply = users
        .get_reply(&data.u, &data.p)
        .ok_or(ServiceError::NotAuth)?;
    Ok(serde_json::to_string(&A {
        t: reply.0,
        r: reply.1,
    })?)
}

pub async fn check_auth(users: &Users, data: A) -> Result<String, ServiceError> {
    let result = users.get_user(&data.t).map_or(false, |u| u.role == data.r);
    Ok(serde_json::to_string(&C { r: result })?)
}

pub fn check(users: &Users, message: ClientMessage) -> Result<Command, ServiceError> {
    let user = users
        .get_user(&message.addon)
        .ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
