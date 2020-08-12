use actix_cors::Cors;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
// use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use log::info;

use rpel::get_pool;

// use auth::bearer_auth_validator;
use db::{delete_name_id, get_list_name, get_name_id, jsonpost, post_name_id};

mod auth;
mod db;
mod error;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    let pool = get_pool();

    std::env::set_var("RUST_LOG", "rugo=info");
    env_logger::init();

    info!("Listening on: {}", addr);

    HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(bearer_auth_validator);
        App::new()
            .data(pool.clone())
            // .wrap(auth)
            .wrap(
                Cors::new()
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .wrap(Compress::default())
            // .data(web::JsonConfig::default().limit(4096))
            // .service(web::resource("/api/go/check").route(web::get().to(check)))
            // .service(web::resource("/api/go/login").route(web::post().to(login)))
            // .service(web::resource("/api/go/logout").route(web::to(logout)))
            // .service(
            //     web::resource("/api/go/{name}/{command}").route(web::get().to(get_name_command)),
            // )
            .service(web::resource("/api/go/json").route(web::post().to(jsonpost)))
            .service(
                web::resource("/api/go/item/{name}/{id}")
                    .route(web::get().to(get_name_id))
                    .route(web::post().to(post_name_id))
                    .route(web::delete().to(delete_name_id)),
            )
            .service(web::resource("/api/go/list/{name}").route(web::get().to(get_list_name)))
    })
    .bind(&addr)?
    .run()
    .await
}
