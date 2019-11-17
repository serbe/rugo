use std::io;
use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use auth::{check, login, logout};
use db::{
    delete_name_id, get_manager, get_name_children, get_name_command, get_name_id, post_name_id,
};
// use db::test_post_name_id;

mod auth;
mod certificate;
mod company;
mod contact;
mod db;
mod department;
mod education;
mod email;
mod kind;
mod phone;
mod post;
mod practice;
mod rank;
mod scope;
mod select;
mod siren;
mod siren_type;
mod tcc;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(), &r, stream);
    println!("{:?}", res.as_ref().unwrap());
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

impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
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

            ctx.ping("");
        });
    }
}

fn main() -> io::Result<()> {
    let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let manager = get_manager();
    let pool = r2d2::Pool::new(manager).expect("error create r2d2 pool");
    let sys = actix_rt::System::new("rugo");

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-example")
                    .secure(false),
            ))
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .service(web::resource("/api/go/check").route(web::get().to(check)))
            .service(web::resource("/api/go/login").route(web::post().to(login)))
            .service(web::resource("/api/go/logout").route(web::to(logout)))
            .service(
                web::resource("/api/go/{name}/{command}")
                    .route(web::get().to_async(get_name_command)),
            )
            .service(
                web::resource("/api/go/{name}/item/{id}")
                    .route(web::get().to_async(get_name_id))
                    .route(web::post().to_async(post_name_id))
                    .route(web::delete().to_async(delete_name_id)),
            )
            .service(
                web::resource("/api/go/{name}/list/{children}/{id}")
                    .route(web::get().to_async(get_name_children)),
            )
        // .service(
        //     web::resource("/api/go/{name}/test/{id}").route(web::post().to(test_post_name_id)),
        // )
    })
    .bind("127.0.0.1:9090")?
    .start();

    sys.run()
}
