// use actix_identity::{CookieIdentityPolicy, IdentityService};
// use actix_session::{Session, CookieSession};
use actix_web::{middleware, web, App, HttpServer};
use std::io;

use db::{get_manager, name_command, name_children, name_id};

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
    let manager = get_manager();
    let pool = r2d2::Pool::new(manager).unwrap();
    let sys = actix_rt::System::new("rugo");
    // let secret_key = dotenv::var("SECRET_KEY").unwrap();
    // let cookie_secret = base64::decode(&secret_key).unwrap();

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            // .wrap(
            //   CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
            //         .secure(false)
            //  )
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(secret_key.as_bytes())
            //         .name("auth")
            //         .path("/")
            //         .domain("localhost")
            //         .max_age_time(chrono::Duration::days(1))
            //         .secure(false), // this can only be true if you have https
            // ))
            // limit the maximum amount of data that server will accept
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::resource("/api/go/{name}/{command}").route(web::get().to_async(name_command)),
            )
            .service(web::resource("/api/go/{name}/item/{id}").route(web::get().to_async(name_id)))
            .service(
                web::resource("/api/go/{name}/list/{children}/{id}")
                    .route(web::get().to_async(name_children)),
            )
    })
    .bind("127.0.0.1:9090")?
    .start();

    sys.run()
}
