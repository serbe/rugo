use std::io;

use actix::Actor;
use actix_web::{middleware, web, App, HttpServer};

// use auth::{check, login, logout};
use server::Server;
use session::route;

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
    let path = dotenv::var("WS_PATH").expect("WS_PATH must be set");

    let server = Server::default().start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource(path).route(web::get().to(route)))
    })
    .bind(addr)?
    .run()
    .await
}
