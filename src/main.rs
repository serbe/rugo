use std::io;

use actix::Actor;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use auth::{login, validator};
use server::Server;
use session::wsroute;

mod auth;
mod db;
mod error;
mod server;
mod session;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");

    env_logger::init();

    // let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    let login_path = dotenv::var("LOGIN_PATH").expect("LOGIN_PATH must be set");
    let ws_path = dotenv::var("WS_PATH").expect("WS_PATH must be set");

    let server = Server::default().start();

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .data(server.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .service(web::resource(&login_path).route(web::post().to(login)))
            .service(web::resource(&ws_path).route(web::get().to(wsroute)))
    })
    .bind(addr)?
    .run()
    .await
}
