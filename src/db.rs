// use actix_session::{Session};
use actix_identity::{Identity};
use actix_web::{web, Error, HttpResponse};
use dotenv::dotenv;
use futures::Future;
use postgres::Connection;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Value::Null};
use std::env;

use crate::certificate::{Certificate, CertificateList};
use crate::company::{Company, CompanyList};
use crate::contact::{Contact, ContactList};
use crate::department::{Department, DepartmentList};
use crate::education::{Education, EducationList, EducationShort};
use crate::kind::{Kind, KindList};
use crate::post::{Post, PostList};
use crate::practice::{Practice, PracticeList, PracticeShort};
use crate::rank::{Rank, RankList};
use crate::scope::{Scope, ScopeList};
use crate::select::SelectItem;
use crate::siren::{Siren, SirenList};
use crate::siren_type::{SirenType, SirenTypeList};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DBResult {
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
    Siren(Siren),
    SirenList(Vec<SirenList>),
    SirenType(SirenType),
    SirenTypeList(Vec<SirenTypeList>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestStruct {
    value: Value,
}

fn get_connurl() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_manager() -> PostgresConnectionManager {
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
        "company" => Ok(DBResult::Company(Company::get(conn, id)?)),
        "contact" => Ok(DBResult::Contact(Contact::get(conn, id)?)),
        "department" => Ok(DBResult::Department(Department::get(conn, id)?)),
        "education" => Ok(DBResult::Education(Education::get(conn, id)?)),
        "kind" => Ok(DBResult::Kind(Kind::get(conn, id)?)),
        "post" => Ok(DBResult::Post(Post::get(conn, id)?)),
        "practice" => Ok(DBResult::Practice(Practice::get(conn, id)?)),
        "rank" => Ok(DBResult::Rank(Rank::get(conn, id)?)),
        "scope" => Ok(DBResult::Scope(Scope::get(conn, id)?)),
        "siren" => Ok(DBResult::Siren(Siren::get(conn, id)?)),
        "siren_type" => Ok(DBResult::SirenType(SirenType::get(conn, id)?)),
        _ => Err("bad path".to_string()),
    }
}

fn post_item(
    conn: &Connection,
    name: &str,
    id: i64,
    params: web::Json<DBResult>,
) -> Result<DBResult, String> {
    match (name, params.into_inner()) {
        ("certificate", DBResult::Certificate(item)) => {
            Ok(DBResult::Certificate(Certificate::post(conn, id, item)?))
        }
        ("company", DBResult::Company(item)) => {
            Ok(DBResult::Company(Company::post(conn, id, item)?))
        }
        ("contact", DBResult::Contact(item)) => {
            Ok(DBResult::Contact(Contact::post(conn, id, item)?))
        }
        ("department", DBResult::Department(item)) => {
            Ok(DBResult::Department(Department::post(conn, id, item)?))
        }
        ("education", DBResult::Education(item)) => {
            Ok(DBResult::Education(Education::post(conn, id, item)?))
        }
        ("kind", DBResult::Kind(item)) => Ok(DBResult::Kind(Kind::post(conn, id, item)?)),
        ("post", DBResult::Post(item)) => Ok(DBResult::Post(Post::post(conn, id, item)?)),
        ("practice", DBResult::Practice(item)) => {
            Ok(DBResult::Practice(Practice::post(conn, id, item)?))
        }
        ("rank", DBResult::Rank(item)) => Ok(DBResult::Rank(Rank::post(conn, id, item)?)),
        ("scope", DBResult::Scope(item)) => Ok(DBResult::Scope(Scope::post(conn, id, item)?)),
        ("siren", DBResult::Siren(item)) => Ok(DBResult::Siren(Siren::post(conn, id, item)?)),
        ("siren_type", DBResult::SirenType(item)) => {
            Ok(DBResult::SirenType(SirenType::post(conn, id, item)?))
        }
        _ => Err(format!("bad path {}", name)),
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

pub fn get_name_children(
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

pub fn get_name_command(
    _identity: Identity,
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // if let Ok(Some(count)) = session.get::<i32>("counter") {
    //     println!("SESSION value: {}", count);
    //     let _ = session.set("counter", count+1);
    // } else {
    //     let _ = session.set("counter", 1);
    // };
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

pub fn get_name_id(
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

pub fn post_name_id(
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, i64)>,
    params: web::Json<DBResult>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let conn = db.get().unwrap();
        post_item(&conn, &path.0, path.1, params)
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

pub fn test_post_name_id(
    _db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, i64)>,
    params: web::Json<TestStruct>,
) -> HttpResponse {
    let values = params.into_inner();
    println!("{} {} {:?}", path.0, path.1, values);
    HttpResponse::Ok().json(json!({
        "data": values,
        "error": Null,
        "ok": true
    }))
}
