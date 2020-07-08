use std::io;
use std::time::{Duration, Instant};

use actix::{Actor, AsyncContext, StreamHandler};
// use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use anyhow::{anyhow, Error};

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
mod server;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    let pool = get_pool();

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");

    // let server = WsServer::new(move || {
    //     App::new()
    //         .data(pool.clone())
    //         .wrap(middleware::Logger::default())
    //         .service(web::resource("/ws/").route(web::get().to(ws_index)))
    //         // .wrap(IdentityService::new(
    //         //     CookieIdentityPolicy::new(&[0; 32])
    //         //         .name("auth-example")
    //         //         .secure(false),
    //         // ))
    //         // .data(web::JsonConfig::default().limit(4096))
    //         // .service(web::resource("/ws/").route(web::get().to(ws_index)))
    //         // .service(web::resource("/api/go/check").route(web::get().to(check)))
    //         // .service(web::resource("/api/go/login").route(web::post().to(login)))
    //         // .service(web::resource("/api/go/logout").route(web::to(logout)))
    //         // .service(
    //         //     web::resource("/api/go/{name}/{command}").route(web::get().to(get_name_command)),
    //         // )
    //         // .service(
    //         //     web::resource("/api/go/{name}/item/{id}")
    //         //         .route(web::get().to(get_name_id))
    //         //         // .route(web::post().to(post_name_id))
    //         //         .route(web::delete().to(delete_name_id)),
    //         // )
    //         // .service(
    //         //     web::resource("/api/go/{name}/list/{children}/{id}")
    //         //         .route(web::get().to(get_name_children)),
    //         // )
    //     // .service(
    //     //     web::resource("/api/go/{name}/test/{id}").route(web::post().to(test_post_name_id)),
    //     // )
    // })
    // .bind("127.0.0.1:9090")?
    // .run();

    // server.await
    Ok(())
}
