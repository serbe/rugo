use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
use crate::services::{ClientMessage, Command, ServerMessage};
use crate::users::Users;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientAuthRequest {
    u: String,
    p: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub t: String,
    pub r: i64,
}

#[derive(Serialize)]
pub struct Check {
    pub r: bool,
}

pub async fn login(users: &Users, data: ClientAuthRequest) -> Result<String, ServiceError> {
    let reply = users
        .get_reply(&data.u, &data.p)
        .ok_or(ServiceError::NotAuth);
    Ok(serde_json::to_string(&ServerMessage::from_reply(reply))?)
}

pub async fn check_auth(users: &Users, data: Token) -> Result<String, ServiceError> {
    let check = users.get_user(&data.t).map_or(false, |u| u.role == data.r);
    Ok(serde_json::to_string(&ServerMessage::from_check(check))?)
}

pub fn check(users: &Users, message: ClientMessage) -> Result<Command, ServiceError> {
    let user = users
        .get_user(&message.addon)
        .ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
