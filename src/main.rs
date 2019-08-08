use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::Future;
use postgres::Connection;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value::Null};
use std::io;

use db::get_connurl;

use certificate::{Certificate, CertificateList};
use company::{Company, CompanyList};
use contact::{Contact, ContactList};
use department::{Department, DepartmentList};
use education::{Education, EducationList, EducationShort};
use kind::{Kind, KindList};
use post::{Post, PostList};
use practice::{Practice, PracticeList, PracticeShort};
use rank::{Rank, RankList};
use scope::{Scope, ScopeList};
use select::SelectItem;
use siren::{Siren, SirenList};
use siren_type::{SirenType, SirenTypeList};

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

// mod models;

#[derive(Deserialize, Serialize)]
enum DBResult {
    Certificate(Certificate),
    CertificateList(Vec<CertificateList>),
    Company(Box<Company>),
    CompanyList(Vec<CompanyList>),
    Contact(Box<Contact>),
    ContactList(Vec<ContactList>),
    Department(Department),
    DepartmentList(Vec<DepartmentList>),
    Education(Education),
    EducationList(Vec<EducationList>),
    EducationShort(Vec<EducationShort>),
    Kind(Kind),
    KindList(Vec<KindList>),
    Post(Post),
    PostList(Vec<PostList>),
    Practice(Practice),
    PracticeList(Vec<PracticeList>),
    PracticeShort(Vec<PracticeShort>),
    Rank(Rank),
    RankList(Vec<RankList>),
    Scope(Scope),
    ScopeList(Vec<ScopeList>),
    SelectItem(Vec<SelectItem>),
    Siren(Box<Siren>),
    SirenList(Vec<SirenList>),
    SirenType(SirenType),
    SirenTypeList(Vec<SirenTypeList>),
}

fn get_manager() -> PostgresConnectionManager {
    let conn_url = get_connurl();
    PostgresConnectionManager::new(conn_url.clone(), r2d2_postgres::TlsMode::None)
        .unwrap_or_else(|_| panic!("Error connection manager to {}", conn_url))
}

fn get_list(conn: &Connection, name: &str, command: &str) -> Result<DBResult, String> {
    match (name, command) {
        ("certificate", "list") => Ok(DBResult::CertificateList(CertificateList::get_all(conn)?)),
        ("company", "list") => Ok(DBResult::CompanyList(CompanyList::get_all(conn)?)),
        ("company", "select") => Ok(DBResult::SelectItem(SelectItem::company_all(conn)?)),
        ("contact", "list") => Ok(DBResult::ContactList(ContactList::get_all(conn)?)),
        ("contact", "select") => Ok(DBResult::SelectItem(SelectItem::contact_all(conn)?)),
        ("department", "list") => Ok(DBResult::DepartmentList(DepartmentList::get_all(conn)?)),
        ("department", "select") => Ok(DBResult::SelectItem(SelectItem::department_all(conn)?)),
        ("education", "list") => Ok(DBResult::EducationList(EducationList::get_all(conn)?)),
        ("education", "near") => Ok(DBResult::EducationShort(EducationShort::get_near(conn)?)),
        ("kind", "list") => Ok(DBResult::KindList(KindList::get_all(conn)?)),
        ("kind", "select") => Ok(DBResult::SelectItem(SelectItem::kind_all(conn)?)),
        ("post", "list") => Ok(DBResult::PostList(PostList::get_all(conn)?)),
        ("post", "select") => Ok(DBResult::SelectItem(SelectItem::post_all(conn, false)?)),
        ("post_go", "select") => Ok(DBResult::SelectItem(SelectItem::post_all(conn, true)?)),
        ("practice", "list") => Ok(DBResult::PracticeList(PracticeList::get_all(conn)?)),
        ("practice", "near") => Ok(DBResult::PracticeShort(PracticeShort::get_near(conn)?)),
        ("rank", "list") => Ok(DBResult::RankList(RankList::get_all(conn)?)),
        ("rank", "select") => Ok(DBResult::SelectItem(SelectItem::rank_all(conn)?)),
        ("scope", "list") => Ok(DBResult::ScopeList(ScopeList::get_all(conn)?)),
        ("scope", "select") => Ok(DBResult::SelectItem(SelectItem::scope_all(conn)?)),
        ("siren", "list") => Ok(DBResult::SirenList(SirenList::get_all(conn)?)),
        ("siren_type", "list") => Ok(DBResult::SirenTypeList(SirenTypeList::get_all(conn)?)),
        ("siren_type", "select") => Ok(DBResult::SelectItem(SelectItem::siren_type_all(conn)?)),
        _ => Err("bad path".to_string()),
    }
}

fn get_item(conn: &Connection, name: &str, id: i64) -> Result<DBResult, String> {
    match name {
        "certificate" => Ok(DBResult::Certificate(Certificate::get(conn, id)?)),
        "company" => Ok(DBResult::Company(Box::new(Company::get(conn, id)?))),
        "contact" => Ok(DBResult::Contact(Box::new(Contact::get(conn, id)?))),
        "department" => Ok(DBResult::Department(Department::get(conn, id)?)),
        "education" => Ok(DBResult::Education(Education::get(conn, id)?)),
        "kind" => Ok(DBResult::Kind(Kind::get(conn, id)?)),
        "post" => Ok(DBResult::Post(Post::get(conn, id)?)),
        "practice" => Ok(DBResult::Practice(Practice::get(conn, id)?)),
        "rank" => Ok(DBResult::Rank(Rank::get(conn, id)?)),
        "scope" => Ok(DBResult::Scope(Scope::get(conn, id)?)),
        "siren" => Ok(DBResult::Siren(Box::new(Siren::get(conn, id)?))),
        "siren_type" => Ok(DBResult::SirenType(SirenType::get(conn, id)?)),
        _ => Err("bad path".to_string()),
    }
}

fn get_children(
    conn: &Connection,
    name: &str,
    children: &str,
    id: i64,
) -> Result<DBResult, String> {
    match (name, children) {
        // "certificate" => Ok(DBResult::Certificate(Certificate::get(conn, id)?)),
        ("company", "practice") => Ok(DBResult::PracticeList(PracticeList::get_by_company(
            conn, id,
        )?)),
        // "contact" => Ok(DBResult::Contact(Box::new(Contact::get(conn, id)?))),
        // "department" => Ok(DBResult::Department(Department::get(conn, id)?)),
        // "education" => Ok(DBResult::Education(Education::get(conn, id)?)),
        // "kind" => Ok(DBResult::Kind(Kind::get(conn, id)?)),
        // "post" => Ok(DBResult::Post(Post::get(conn, id)?)),
        // "practice" => Ok(DBResult::Practice(Practice::get(conn, id)?)),
        _ => Err("bad path".to_string()),
    }
}

fn name_children(
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, String, i64)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let conn = db.get().unwrap();
        get_children(&conn, &path.0, &path.1, path.2)
    })
    .then(|res| match res {
        Ok(db_result) => Ok(HttpResponse::Ok().json(json!({
            "data": db_result,
            "error": Null,
            "ok": true
        }))),
        Err(err) => {
            println!("{}", err);
            Ok(HttpResponse::InternalServerError().into())
        }
    })
}

fn name_command(
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let conn = db.get().unwrap();
        get_list(&conn, &path.0, &path.1)
    })
    .then(|res| match res {
        Ok(db_result) => Ok(HttpResponse::Ok().json(json!({
            "data": db_result,
            "error": Null,
            "ok": true
        }))),
        Err(err) => {
            println!("{}", err);
            Ok(HttpResponse::InternalServerError().into())
        }
    })
}

fn name_id(
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, i64)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let conn = db.get().unwrap();
        get_item(&conn, &path.0, path.1)
    })
    .then(|res| match res {
        Ok(db_result) => Ok(HttpResponse::Ok().json(json!({
            "data": db_result,
            "error": Null,
            "ok": true
        }))),
        Err(err) => {
            println!("{}", err);
            Ok(HttpResponse::InternalServerError().into())
        }
    })
}

fn main() -> io::Result<()> {
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
                CookieIdentityPolicy::new("SECRET_KEY".as_bytes())
                    .name("auth")
                    .path("/")
                    .domain("localhost")
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
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
