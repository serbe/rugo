use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::Deserialize;
use ws::{Handler, Handshake, Message, Result as WSResult, Sender};

// use crate::error::Result;
use crate::db::{
    delete_item, get_item, get_list, get_near, get_select, insert_item, update_item, DBItem,
};

#[derive(Deserialize)]
pub struct Item {
    id: i64,
    name: String,
}

#[derive(Deserialize)]
enum Command {
    Item(i64),
    List,
    Near,
    Select,
}

#[derive(Deserialize)]
struct GetRequest {
    pub command: Command,
    pub name: String,
}

#[derive(Deserialize)]
enum ClientMessage {
    Get(GetRequest),
    Insert(DBItem),
    Update(DBItem),
    Delete(Item),
}

pub struct Server {
    pub out: Sender,
    pub pool: Pool<PostgresConnectionManager>,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> WSResult<()> {
        println!("Server open connection '{:?}'", shake.remote_addr());
        // schedule a timeout to send a ping every 5 seconds
        // self.out.timeout(5_000, PING)?;
        // schedule a timeout to close the connection if there is no activity for 30 seconds
        // self.out.timeout(30_000, EXPIRE)
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> WSResult<()> {
        let json_string = msg.as_text()?;
        let message: ClientMessage =
            serde_json::from_str(json_string).map_err(|err| ws::Error::from(Box::new(err)))?;

        // println!("Received {:?}", message);
        let conn = self
            .pool
            .get()
            .map_err(|err| ws::Error::from(Box::new(err)))?;

        let _ = match message {
            ClientMessage::Get(request) => match request.command {
                Command::Item(id) => get_item(&conn, id, request.name),
                Command::List => get_list(&conn, request.name),
                Command::Near => get_near(&conn, request.name),
                Command::Select => get_select(&conn, request.name),
            },
            ClientMessage::Insert(item) => insert_item(&conn, item),
            ClientMessage::Update(item) => update_item(&conn, item),
            ClientMessage::Delete(item) => delete_item(&conn, item.id, item.name),
        };

        Ok(self.out.send(Message::Text(format!("echo {}", "gg")))?) // simple echo
    }

    fn on_error(&mut self, err: ws::Error) {
        // Shutdown on any error
        println!("Shutting down server for error: {}", err);
        self.out.shutdown().unwrap();
    }
}

// fn to_ws_error(msg: String) -> ws::Error {
//     ws::Error::
// }
