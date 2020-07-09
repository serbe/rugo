use actix::{spawn, Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use deadpool_postgres::Pool;
use serde_json::json;
// use log::info;

use crate::db::ws_text;

struct MyWs {
    pool: Pool,
}

pub struct WebData {
    pub pool: Pool,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    async fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let fut = async move {
                    let msg = ws_text(self.pool.clone(), text).await;
                    match serde_json::to_string(&msg) {
                        Ok(txt) => ctx.text(txt),
                        Err(err) => (),
                    }
                };
                spawn(fut);
            }
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
