use std::net::SocketAddr;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use anyhow::{Error, Result};
use deadpool_postgres::Pool;
use dotenv::dotenv;
use futures::StreamExt;
use futures_util::SinkExt;
use log::{error, info};
use serde_json;
use tokio::net::{TcpListener, TcpStream};

use rpel::get_pool;

// use auth::{check, login, logout};
use db::{get_name_children, get_name_command, get_name_id, post_name_id, Command};

// mod auth;
mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    let pool = get_pool();

    std::env::set_var("RUST_LOG", "rugo=info");
    env_logger::init();

    info!("Listening on: {}", addr);

    HttpServer::new(|| {
        App::new()
            .data(pool.clone())
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .wrap(Compress::default())
            .data(web::JsonConfig::default().limit(4096))
            // .service(web::resource("/api/go/check").route(web::get().to(check)))
            // .service(web::resource("/api/go/login").route(web::post().to(login)))
            // .service(web::resource("/api/go/logout").route(web::to(logout)))
            .service(
                web::resource("/api/go/{name}/{command}").route(web::get().to(get_name_command)),
            )
            .service(
                web::resource("/api/go/{name}/item/{id}")
                    .route(web::get().to(get_name_id))
                    .route(web::post().to(post_name_id))
                    .route(web::delete().to(delete_name_id)),
            )
            .service(
                web::resource("/api/go/{name}/list/{children}/{id}")
                    .route(web::get().to(get_name_children)),
            )
    })
    .bind(&addr)?
    .run()
    .await
}
