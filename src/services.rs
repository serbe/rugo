use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
// use log::info;

use crate::auth::check;
use crate::dbo::{delete_item, get_item, get_list, insert_item, update_item, DBObject};
use crate::error::ServiceError;
use crate::users::{user_cmd, UserObject, USERS};

#[derive(Deserialize)]
pub struct ClientMessage {
    pub command: Command,
    pub addon: String,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub id: i64,
}

#[derive(Deserialize)]
pub enum Object {
    Item(Item),
    List(String),
}

#[derive(Deserialize)]
pub enum Command {
    Get(Object),
    Insert(DBObject),
    Update(DBObject),
    Delete(Item),
    // User(UserObject),
}

#[derive(Serialize)]
pub struct WsMsg {
    pub command: String,
    pub name: String,
    pub object: DBObject,
    pub error: String,
}

impl WsMsg {
    pub fn from_dbo(command: &str, name: String, dbo: Result<DBObject, ServiceError>) -> WsMsg {
        match dbo {
            Ok(object) => WsMsg {
                command: command.to_string(),
                name,
                object,
                error: String::new(),
            },
            Err(err) => WsMsg {
                command: command.to_string(),
                name,
                object: DBObject::Null,
                error: err.to_string(),
            },
        }
    }
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

pub async fn jsonpost(
    db: Pool,
    msg: ClientMessage,
) -> Result<WsMsg, ServiceError> {
    // let msg: ClientMessage = params.into_inner();
    let cmd = check(msg)?;
    let client = db.get().await?;
    let msg = match cmd {
        Command::Get(object) => match object {
            Object::Item(item) => {
                WsMsg::from_dbo("Get", item.name.clone(), get_item(&item, &client).await)
            }
            Object::List(obj) => WsMsg::from_dbo("Get", obj.clone(), get_list(&obj, &client).await),
        },
        Command::Insert(dbobject) => WsMsg::from_dbo(
            "Insert",
            dbobject.name(),
            Ok(insert_item(dbobject, &client)
                .await
                .map(|_| DBObject::Null)?),
        ),
        Command::Update(dbobject) => WsMsg::from_dbo(
            "Update",
            dbobject.name(),
            Ok(update_item(dbobject, &client)
                .await
                .map(|_| DBObject::Null)?),
        ),
        Command::Delete(item) => WsMsg::from_dbo(
            "Delete",
            item.name.clone(),
            Ok(delete_item(&item, &client).await.map(|_| DBObject::Null)?),
        ),
        // Command::User(obj) => return user_cmd(obj, &client).await,
    };
    Ok(msg)
}
