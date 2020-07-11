use std::time::{Duration, Instant};

use actix::{
    fut, Actor, ActorContext, ActorFuture, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::info;

use crate::db::{DBObject, DB};
use crate::error::ServiceError;
use crate::server::{ClientMessage, Connect, Disconnect, Join, Msg, Server};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let db = DB::new(srv.get_ref().clone()).start();
    ws::start(
        Session {
            id: 0,
            hb: Instant::now(),
            server: srv.get_ref().clone(),
            db,
        },
        &req,
        stream,
    )
}

struct Session {
    id: usize,
    hb: Instant,
    server: Addr<Server>,
    db: Addr<DB>,
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.server
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.server.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<Msg> for Session {
    type Result = Result<DBObject, ServiceError>;

    fn handle(&mut self, _msg: Msg, _ctx: &mut Self::Context) -> Self::Result {
        Ok(DBObject::Null)
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        info!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(msg) => {
                self.db
                    .send(Msg(msg))
                    .into_actor(self)
                    .then(|res, another, ctx| {
                        match res {
                            Ok(_dbo) => info!("get dbo"),
                            _ => info!("Something is wrong"),
                        }
                        fut::ready(())
                    })
                    .wait(ctx)
                // self.addr.send(ClientMessage { id: self.id, msg })
                //     .into_actor(self)
                //     .then(|res, _, ctx| {
                //         match res {
                //             Ok(text) => {
                //                     ctx.text(text);
                //             }
                //             _ => info!("Something is wrong"),
                //         }
                //         fut::ready(())
                //     })
                //     .wait(ctx)
                // .wait(ctx) pauses all events in context,
                // so actor wont receive any new messages until it get list
                // of rooms back
            }
            ws::Message::Binary(_) => info!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl Session {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting!");
                act.server.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}
