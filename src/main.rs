use std::io;

use actix::{Actor, Addr};
use actix_web::{middleware, web, App, HttpServer};
use deadpool_postgres::Pool;

// use auth::{check, login, logout};
// use db::{
//     delete_name_id,
//     get_name_children,
//     get_name_command,
//     get_name_id,
//     //post_name_id,
// };
use server::Server;
use session::route;

use rpel::get_pool;

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
    // let pool = get_pool();
    let server = Server::default().start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/go/ws/").route(web::get().to(route)))
    })
    .bind(addr)?
    .run()
    .await
}
