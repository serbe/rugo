use std::io;

use actix::Actor;
use actix_web::{middleware, web, App, HttpServer};
// use actix_web_httpauth::middleware::HttpAuthentication;

use auth::login;
use db::global_init;
use server::Server;
use session::wsroute;

mod auth;
mod db;
mod error;
// mod redirect;
mod server;
mod session;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");

    env_logger::init();

    global_init().await.unwrap();
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");

    let server = Server::default().start();

    HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(validator);
        App::new()
            .data(server.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // .wrap(auth)
            .service(web::resource("/api/go/login").route(web::post().to(login)))
            .service(web::resource("/api/go").route(web::get().to(wsroute)))
    })
    .bind(addr)?
    .run()
    .await
}
