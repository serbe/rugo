use deadpool_postgres::Pool;
use log::info;
use serde::{Deserialize, Serialize};

use crate::auth::{check, Check, Token};
use crate::dbo::{delete_item, get_item, get_list, insert_item, update_item, DBObject};
use crate::error::ServiceError;
use crate::users::Users;

#[derive(Deserialize)]
pub struct ClientMessage {
    pub id: i64,
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
pub enum MessageData {
    Null,
    DBData(DBObject),
    Token(Token),
    Check(Check),
    ResultInt(i64),
}

#[derive(Serialize)]
pub struct ServerMessage {
    pub id: i64,
    pub command: String,
    pub data: MessageData,
    pub error: String,
}

impl ServerMessage {
    pub fn from_dbo(
        id: i64,
        command: String,
        dbo: Result<DBObject, ServiceError>,
    ) -> ServerMessage {
        match dbo {
            Ok(object) => ServerMessage {
                id,
                command: command,
                data: MessageData::DBData(object),
                error: String::new(),
            },
            Err(err) => ServerMessage {
                id,
                command: command,
                data: MessageData::Null,
                error: err.to_string(),
            },
        }
    }

    pub fn from_i64(id: i64, command: String, dbo: Result<i64, ServiceError>) -> ServerMessage {
        match dbo {
            Ok(object) => ServerMessage {
                id,
                command: command,
                data: MessageData::ResultInt(object),
                error: String::new(),
            },
            Err(err) => ServerMessage {
                id,
                command: command,
                data: MessageData::Null,
                error: err.to_string(),
            },
        }
    }

    pub fn from_reply(id: i64, reply: Result<(String, i64), ServiceError>) -> ServerMessage {
        info!("reply {:?}", reply);
        match reply {
            Ok(object) => ServerMessage {
                id,
                command: "Token".to_string(),
                data: MessageData::Token(Token {
                    t: object.0,
                    r: object.1,
                }),
                error: String::new(),
            },
            Err(err) => ServerMessage {
                id,
                command: "Token".to_string(),
                data: MessageData::Null,
                error: err.to_string(),
            },
        }
    }

    pub fn from_check(id: i64, check: bool) -> ServerMessage {
        ServerMessage {
            id,
            command: "Check".to_string(),
            data: MessageData::Check(Check { r: check }),
            error: String::new(),
        }
    }

    // pub fn from_rsm(id: i64, rsm: Result<ServerMessage, ServiceError>) -> ServerMessage {
    //     match rsm {
    //         Ok(sm) => sm,
    //         Err(err) => ServerMessage {
    //             id,
    //             command: String::new(),
    //             data: MessageData::Null,
    //             error: err.to_string(),
    //         }
    //     }
    // }
}

pub async fn get_response(
    users: &Users,
    msg: ClientMessage,
    db: Pool,
) -> Result<String, ServiceError> {
    let id = msg.id;
    let cmd = check(users, msg)?;
    let client = db.get().await?;
    let server_msg = match cmd {
        Command::Get(object) => match object {
            Object::Item(item) => {
                ServerMessage::from_dbo(id, item.name.clone(), get_item(&item, &client).await)
            }
            Object::List(obj) => {
                ServerMessage::from_dbo(id, obj.clone(), get_list(&obj, &client).await)
            }
        },
        Command::Insert(dbobject) => ServerMessage::from_i64(
            id,
            format!("Insert-{}", dbobject.name()),
            insert_item(dbobject, &client).await,
        ),
        Command::Update(dbobject) => ServerMessage::from_i64(
            id,
            format!("Update-{}", dbobject.name()),
            update_item(dbobject, &client).await,
        ),
        Command::Delete(item) => ServerMessage::from_i64(
            id,
            format!("Delete-{}", item.name),
            delete_item(&item, &client).await,
        ),
        // Command::User(obj) => ServerMessage::from_rsm(msg.id, user_cmd(msg.id, obj, &client).await),
    };
    Ok(serde_json::to_string(&server_msg)?)
}
