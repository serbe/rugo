use std::io;
use std::time::{Duration, Instant};

use actix::{Actor, AsyncContext, StreamHandler};
// use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use anyhow::Error;

// use auth::{check, login, logout};
// use db::{
//     delete_name_id,
//     get_name_children,
//     get_name_command,
//     get_name_id,
//     //post_name_id,
// };
// use db::test_post_name_id;

use rpel::get_pool;

// mod auth;
// mod db;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(), &r, stream);
    println!("{:?}", res);
    res
}

struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let pool = get_pool();
    let sys = actix_rt::System::new("rugo");

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(&[0; 32])
            //         .name("auth-example")
            //         .secure(false),
            // ))
            // .data(web::JsonConfig::default().limit(4096))
            // .service(web::resource("/ws/").route(web::get().to(ws_index)))
            // .service(web::resource("/api/go/check").route(web::get().to(check)))
            // .service(web::resource("/api/go/login").route(web::post().to(login)))
            // .service(web::resource("/api/go/logout").route(web::to(logout)))
            // .service(
            //     web::resource("/api/go/{name}/{command}").route(web::get().to(get_name_command)),
            // )
            // .service(
            //     web::resource("/api/go/{name}/item/{id}")
            //         .route(web::get().to(get_name_id))
            //         // .route(web::post().to(post_name_id))
            //         .route(web::delete().to(delete_name_id)),
            // )
            // .service(
            //     web::resource("/api/go/{name}/list/{children}/{id}")
            //         .route(web::get().to(get_name_children)),
            // )
        // .service(
        //     web::resource("/api/go/{name}/test/{id}").route(web::post().to(test_post_name_id)),
        // )
    })
    .bind("127.0.0.1:9090")?
    .run();

    server.await
}
