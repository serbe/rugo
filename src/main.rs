use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use std::io;

use auth::{check, login, logout};
use db::{
    delete_name_id, get_manager, get_name_children, get_name_command, get_name_id, post_name_id,
    test_post_name_id,
};

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

fn main() -> io::Result<()> {
    let _secret_key = dotenv::var("SECRET_KEY").unwrap();
    let manager = get_manager();
    let pool = r2d2::Pool::new(manager).unwrap();
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
            .service(
                web::resource("/api/go/{name}/test/{id}").route(web::post().to(test_post_name_id)),
            )
    })
    .bind("127.0.0.1:9090")?
    .start();

    sys.run()
}
