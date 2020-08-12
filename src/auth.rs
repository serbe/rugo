use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
use crate::services::{ClientMessage, Command};
use crate::users::{get_user, USERS};

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

pub async fn login(data: Auth) -> Result<String, ServiceError> {
    let reply = get_reply(&data.u, &data.p).ok_or(ServiceError::NotAuth)?;
    Ok(serde_json::to_string(&A {
        t: reply.0,
        r: reply.1,
    })?)
}

pub fn get_reply(username: &str, userkey: &str) -> Option<(String, i64)> {
    let mutex = USERS.get()?;
    let users = mutex.lock().ok()?;
    let reply = users
        .iter()
        .find(|(_key, user)| user.name == username && user.key == userkey)
        .map(|(key, user)| (key, user.role))?;
    Some((reply.0.clone(), reply.1))
}

pub async fn check_auth(data: A) -> Result<String, ServiceError> {
    let result = get_user(&data.t)
        .map(|u| u.role == data.r)
        .ok_or(ServiceError::NotAuth)?;
    Ok(serde_json::to_string(&C { r: result })?)
}

pub fn check(message: ClientMessage) -> Result<Command, ServiceError> {
    let user = get_user(&message.addon).ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
