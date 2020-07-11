use std::collections::HashMap;

use actix::{Actor, Context, Handler, Message, Recipient, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use deadpool_postgres::Pool;
use log::info;
use rand::{self, rngs::ThreadRng, Rng};
// use serde_json::json;

use crate::db::DBObject;
use crate::error::ServiceError;

pub struct Msg(pub String);

impl Message for Msg {
    type Result = Result<DBObject, ServiceError>;
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Msg>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
}

pub struct Server {
    sessions: HashMap<usize, Recipient<Msg>>,
    rng: ThreadRng,
}

impl Default for Server {
    fn default() -> Server {
        Server {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl Server {
    fn send_message(&self, message: &str, id: usize) {
        if let Some(addr) = self.sessions.get(&id) {
            let _ = addr.do_send(Msg(message.to_owned()));
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        info!("Someone joined");

        // self.send_message(&"Main".to_owned(), "Someone joined", 0);

        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        id
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        info!("Someone disconnected");

        self.sessions.remove(&msg.id);
    }
}

impl Handler<ClientMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.msg.as_str(), msg.id);
    }
}

impl Handler<Join> for Server {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id } = msg;

        self.send_message("Someone connected", id);
    }
}

struct MyWs {
    pool: Pool,
}

pub struct WebData {
    pub pool: Pool,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<Msg> for MyWs {
    type Result = Result<DBObject, ServiceError>;

    fn handle(&mut self, msg: Msg, ctx: &mut Self::Context) -> Self::Result {
        Ok(DBObject::Null)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            // let fut = async move {
            //     let msg = ws_text(self.pool.clone(), text).await;
            //     match serde_json::to_string(&msg) {
            //         Ok(txt) => ctx.text(txt),
            //         Err(err) => (),
            //     }
            // };
            // spawn(fut);
            // ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn ws_index(
    data: web::Data<WebData>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(
        MyWs {
            pool: data.pool.clone(),
        },
        &req,
        stream,
    )
}
