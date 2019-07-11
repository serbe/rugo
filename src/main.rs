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
use postgres::Connection;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value::Null};

use repdb::get_connurl;

use repdb::certificate::{Certificate, CertificateList};
use repdb::company::{Company, CompanyList};
use repdb::contact::{Contact, ContactList};
use repdb::department::{Department, DepartmentList};
use repdb::education::{Education, EducationList, EducationShort};
use repdb::kind::{Kind, KindList};
use repdb::post::{Post, PostList};
use repdb::practice::{Practice, PracticeList, PracticeShort};
use repdb::rank::{Rank, RankList};
use repdb::scope::{Scope, ScopeList};
use repdb::select::SelectItem;
use repdb::siren::SirenList;
use repdb::siren_type::SirenTypeList;

#[derive(Deserialize, Serialize)]
enum DBResult {
    Certificate(Certificate),
    CertificateList(Vec<CertificateList>),
    Company(Company),
    CompanyList(Vec<CompanyList>),
    Contact(Contact),
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
    SirenList(Vec<SirenList>),
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
        ("department", "list") => Ok(DBResult::DepartmentList(DepartmentList::get_all(conn)?)),
        ("department", "select") => Ok(DBResult::SelectItem(SelectItem::department_all(conn)?)),
        ("education", "list") => Ok(DBResult::EducationList(EducationList::get_all(conn)?)),
        ("education", "near") => Ok(DBResult::EducationShort(EducationShort::get_near(conn)?)),
        ("kind", "list") => Ok(DBResult::KindList(KindList::get_all(conn)?)),
        ("post", "list") => Ok(DBResult::PostList(PostList::get_all(conn)?)),
        ("post", "select") => Ok(DBResult::SelectItem(SelectItem::post_all(conn, false)?)),
        ("postgo", "select") => Ok(DBResult::SelectItem(SelectItem::post_all(conn, true)?)),
        ("practice", "list") => Ok(DBResult::PracticeList(PracticeList::get_all(conn)?)),
        ("practice", "near") => Ok(DBResult::PracticeShort(PracticeShort::get_near(conn)?)),
        ("rank", "list") => Ok(DBResult::RankList(RankList::get_all(conn)?)),
        ("rank", "select") => Ok(DBResult::SelectItem(SelectItem::rank_all(conn)?)),
        ("scope", "list") => Ok(DBResult::ScopeList(ScopeList::get_all(conn)?)),
        ("siren", "list") => Ok(DBResult::SirenList(SirenList::get_all(conn)?)),
        ("sirentype", "list") => Ok(DBResult::SirenTypeList(SirenTypeList::get_all(conn)?)),
        _ => Err("bad path".to_string()),
    }
}

fn get_item(conn: &Connection, name: &str, id: &str) -> Result<DBResult, String> {
    let id = id
        .parse::<i64>()
        .map_err(|_| format!("parse {} as i64", id))?;
    match name {
        "certificate" => Ok(DBResult::Certificate(Certificate::get(conn, id)?)),
        "company" => Ok(DBResult::Company(Company::get(conn, id)?)),
        "contact" => Ok(DBResult::Contact(Contact::get(conn, id)?)),
        "department" => Ok(DBResult::Department(Department::get(conn, id)?)),
        "education" => Ok(DBResult::Education(Education::get(conn, id)?)),
        "kind" => Ok(DBResult::Kind(Kind::get(conn, id)?)),
        "post" => Ok(DBResult::Post(Post::get(conn, id)?)),
        "practice" => Ok(DBResult::Practice(Practice::get(conn, id)?)),
        _ => Err("bad path".to_string()),
    }
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
    path: web::Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let conn = db.get().unwrap();
        get_item(&conn, &path.0, &path.1)
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

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // .wrap(middleware::Logger::default())
            .service(
                web::resource("/api/go/{name}/{command}").route(web::get().to_async(name_command)),
            )
            .service(web::resource("/api/go/{name}/item/{id}").route(web::get().to_async(name_id)))
        // .route(
        //     "/api/go/practices/near",
        //     web::get().to_async(practices_near),
        // )
        // .route(
        //     "/api/go/contacts/list",
        //     web::get().to_async(contacts_list),
        // )
    })
    .bind("127.0.0.1:9090")?
    .start();

    sys.run()
}
