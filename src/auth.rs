use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::services::{ClientMessage, Command, ServerMessage};
use crate::users::Users;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientAuthRequest {
    i: i64,
    u: String,
    p: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientTokenRequest {
    pub i: i64,
    pub t: String,
    pub r: i64,
}

#[derive(Serialize)]
pub struct Token {
    pub t: String,
    pub r: i64,
}

#[derive(Serialize)]
pub struct Check {
    pub r: bool,
}

pub async fn login(users: &Users, data: ClientAuthRequest) -> Result<String> {
    let reply = users
        .get_reply(&data.u, &data.p)
        .ok_or_else(|| anyhow!("NotAuth"));
    Ok(serde_json::to_string(&ServerMessage::from_reply(
        data.i, reply,
    ))?)
}

pub async fn check_auth(users: &Users, data: ClientTokenRequest) -> Result<String> {
    let check = users.get_user(&data.t).map_or(false, |u| u.role == data.r);
    Ok(serde_json::to_string(&ServerMessage::from_check(
        data.i, check,
    ))?)
}

pub fn check(users: &Users, message: ClientMessage) -> Result<Command> {
    let user = users
        .get_user(&message.addon)
        .ok_or_else(|| anyhow!("NotAuth"))?;
    user.permissions(message.command)
}
