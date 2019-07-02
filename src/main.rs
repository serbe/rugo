use std::io;

use actix_web::{
    // middleware,
    web,
    App,
    Error,
    HttpResponse,
    HttpServer,
};
use futures::Future;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde_json::json;
use serde_json::Value::Null;

use repdb::get_connurl;
use repdb::practice::PracticeShort;
use repdb::education::EducationShort;

fn get_manager() -> PostgresConnectionManager {
    let conn_url = get_connurl();
    PostgresConnectionManager::new(conn_url.clone(), r2d2_postgres::TlsMode::None)
        .unwrap_or_else(|_| panic!("Error connection manager to {}", conn_url))
}

fn educations_near(
    db: web::Data<Pool<PostgresConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let conn = db.get().unwrap();
        EducationShort::get_near(&conn)
    })
    .then(|res| match res {
        Ok(db_result) => Ok(HttpResponse::Ok().json(json!({
            "data": db_result,
            "error": Null,
            "ok": true
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn practices_near(
    db: web::Data<Pool<PostgresConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let conn = db.get().unwrap();
        PracticeShort::get_near(&conn)
    })
    .then(|res| match res {
        Ok(db_result) => Ok(HttpResponse::Ok().json(json!({
            "data": db_result,
            "error": Null,
            "ok": true
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn main() -> io::Result<()> {
    let manager = get_manager();
    let pool = r2d2::Pool::new(manager).unwrap();
    let sys = actix_rt::System::new("rugo");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // .wrap(middleware::Logger::default())
            .route(
                "/api/go/educations/near",
                web::get().to_async(educations_near),
            )
            .route(
                "/api/go/practices/near",
                web::get().to_async(practices_near),
            )
            // .route(
            //     "/api/go/contacts/list",
            //     web::get().to_async(contacts_list),
            // )
    })
    .bind("127.0.0.1:9090")?
    .start();

    sys.run()
}
