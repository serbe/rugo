use std::io;

use actix_web::{middleware, web, App, HttpServer};

// use auth::{check, login, logout};
// use db::{
//     delete_name_id,
//     get_name_children,
//     get_name_command,
//     get_name_id,
//     //post_name_id,
// };
use server::{ws_index, WebData};

use rpel::get_pool;

mod db;
mod error;
mod server;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    let pool = get_pool();

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");

    HttpServer::new(
        move || {
            App::new()
                .data(WebData { pool: pool.clone() })
                .wrap(middleware::Compress::default())
                .wrap(middleware::Logger::default())
                .service(web::resource("/api/go/ws/").route(web::get().to(ws_index)))
        }, // .wrap(IdentityService::new(
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
    )
    .bind(addr)?
    .run()
    .await
}
