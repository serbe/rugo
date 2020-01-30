use actix_identity::Identity;
use actix_web::{error::BlockingError, web, Error, HttpResponse};
use dotenv::dotenv;
use futures::Future;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Value::Null};

use rpel::certificate::{Certificate, CertificateList};
use rpel::company::{Company, CompanyList};
use rpel::contact::{Contact, ContactList};
use rpel::department::{Department, DepartmentList};
use rpel::education::{Education, EducationList, EducationShort};
use rpel::kind::{Kind, KindList};
use rpel::post::{Post, PostList};
use rpel::practice::{Practice, PracticeList, PracticeShort};
use rpel::rank::{Rank, RankList};
use rpel::scope::{Scope, ScopeList};
use rpel::select::SelectItem;
use rpel::siren::{Siren, SirenList};
use rpel::siren_type::{SirenType, SirenTypeList};

use crate::auth::check_auth;

#[derive(Deserialize, Serialize)]
pub enum DBItem {
    Certificate(Certificate),
    Company(Box<Company>),
    Contact(Box<Contact>),
    Department(Department),
    Education(Education),
    Kind(Kind),
    Post(Post),
    Practice(Practice),
    Rank(Rank),
    Scope(Scope),
    Siren(Box<Siren>),
    SirenType(SirenType),
}

#[derive(Serialize)]
pub enum DBList {
    CertificateList(Vec<CertificateList>),
    CompanyList(Vec<CompanyList>),
    ContactList(Vec<ContactList>),
    DepartmentList(Vec<DepartmentList>),
    EducationList(Vec<EducationList>),
    EducationShort(Vec<EducationShort>),
    KindList(Vec<KindList>),
    PostList(Vec<PostList>),
    PracticeList(Vec<PracticeList>),
    PracticeShort(Vec<PracticeShort>),
    RankList(Vec<RankList>),
    ScopeList(Vec<ScopeList>),
    SelectItem(Vec<SelectItem>),
    SirenList(Vec<SirenList>),
    SirenTypeList(Vec<SirenTypeList>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestStruct {
    value: Value,
}

fn get_connurl() -> String {
    dotenv().ok();
    dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn get_list(conn: &Connection, name: &str, command: &str) -> Result<DBList, String> {
    match (name, command) {
        ("certificate", "list") => Ok(DBList::CertificateList(CertificateList::get_all(conn)?)),
        ("company", "list") => Ok(DBList::CompanyList(CompanyList::get_all(conn)?)),
        ("company", "select") => Ok(DBList::SelectItem(SelectItem::company_all(conn)?)),
        ("contact", "list") => Ok(DBList::ContactList(ContactList::get_all(conn)?)),
        ("contact", "select") => Ok(DBList::SelectItem(SelectItem::contact_all(conn)?)),
        ("department", "list") => Ok(DBList::DepartmentList(DepartmentList::get_all(conn)?)),
        ("department", "select") => Ok(DBList::SelectItem(SelectItem::department_all(conn)?)),
        ("education", "list") => Ok(DBList::EducationList(EducationList::get_all(conn)?)),
        ("education", "near") => Ok(DBList::EducationShort(EducationShort::get_near(conn)?)),
        ("kind", "list") => Ok(DBList::KindList(KindList::get_all(conn)?)),
        ("kind", "select") => Ok(DBList::SelectItem(SelectItem::kind_all(conn)?)),
        ("post", "list") => Ok(DBList::PostList(PostList::get_all(conn)?)),
        ("post", "select") => Ok(DBList::SelectItem(SelectItem::post_all(conn, false)?)),
        ("post_go", "select") => Ok(DBList::SelectItem(SelectItem::post_all(conn, true)?)),
        ("practice", "list") => Ok(DBList::PracticeList(PracticeList::get_all(conn)?)),
        ("practice", "near") => Ok(DBList::PracticeShort(PracticeShort::get_near(conn)?)),
        ("rank", "list") => Ok(DBList::RankList(RankList::get_all(conn)?)),
        ("rank", "select") => Ok(DBList::SelectItem(SelectItem::rank_all(conn)?)),
        ("scope", "list") => Ok(DBList::ScopeList(ScopeList::get_all(conn)?)),
        ("scope", "select") => Ok(DBList::SelectItem(SelectItem::scope_all(conn)?)),
        ("siren", "list") => Ok(DBList::SirenList(SirenList::get_all(conn)?)),
        ("siren_type", "list") => Ok(DBList::SirenTypeList(SirenTypeList::get_all(conn)?)),
        ("siren_type", "select") => Ok(DBList::SelectItem(SelectItem::siren_type_all(conn)?)),
        _ => Err("bad path".to_string()),
    }
}

fn get_item(conn: &Connection, name: &str, id: i64) -> Result<DBItem, String> {
    match name {
        "certificate" => Ok(DBItem::Certificate(Certificate::get(conn, id)?)),
        "company" => Ok(DBItem::Company(Box::new(Company::get(conn, id)?))),
        "contact" => Ok(DBItem::Contact(Box::new(Contact::get(conn, id)?))),
        "department" => Ok(DBItem::Department(Department::get(conn, id)?)),
        "education" => Ok(DBItem::Education(Education::get(conn, id)?)),
        "kind" => Ok(DBItem::Kind(Kind::get(conn, id)?)),
        "post" => Ok(DBItem::Post(Post::get(conn, id)?)),
        "practice" => Ok(DBItem::Practice(Practice::get(conn, id)?)),
        "rank" => Ok(DBItem::Rank(Rank::get(conn, id)?)),
        "scope" => Ok(DBItem::Scope(Scope::get(conn, id)?)),
        "siren" => Ok(DBItem::Siren(Box::new(Siren::get(conn, id)?))),
        "siren_type" => Ok(DBItem::SirenType(SirenType::get(conn, id)?)),
        _ => Err("bad path".to_string()),
    }
}

fn post_item(
    conn: &Connection,
    name: &str,
    id: i64,
    params: web::Json<DBItem>,
) -> Result<DBItem, String> {
    match (name, params.into_inner()) {
        ("certificate", DBItem::Certificate(item)) => {
            Ok(DBItem::Certificate(Certificate::post(conn, id, item)?))
        }
        ("company", DBItem::Company(item)) => {
            Ok(DBItem::Company(Box::new(Company::post(conn, id, *item)?)))
        }
        ("contact", DBItem::Contact(item)) => {
            Ok(DBItem::Contact(Box::new(Contact::post(conn, id, *item)?)))
        }
        ("department", DBItem::Department(item)) => {
            Ok(DBItem::Department(Department::post(conn, id, item)?))
        }
        ("education", DBItem::Education(item)) => {
            Ok(DBItem::Education(Education::post(conn, id, item)?))
        }
        ("kind", DBItem::Kind(item)) => Ok(DBItem::Kind(Kind::post(conn, id, item)?)),
        ("post", DBItem::Post(item)) => Ok(DBItem::Post(Post::post(conn, id, item)?)),
        ("practice", DBItem::Practice(item)) => {
            Ok(DBItem::Practice(Practice::post(conn, id, item)?))
        }
        ("rank", DBItem::Rank(item)) => Ok(DBItem::Rank(Rank::post(conn, id, item)?)),
        ("scope", DBItem::Scope(item)) => Ok(DBItem::Scope(Scope::post(conn, id, item)?)),
        ("siren", DBItem::Siren(item)) => {
            Ok(DBItem::Siren(Box::new(Siren::post(conn, id, *item)?)))
        }
        ("siren_type", DBItem::SirenType(item)) => {
            Ok(DBItem::SirenType(SirenType::post(conn, id, item)?))
        }
        _ => Err(format!("bad path {}", name)),
    }
}

fn delete_item(conn: &Connection, name: &str, id: i64) -> Result<bool, String> {
    match name {
        "certificate" => Ok(Certificate::delete(conn, id)),
        "company" => Ok(Company::delete(conn, id)),
        "contact" => Ok(Contact::delete(conn, id)),
        "department" => Ok(Department::delete(conn, id)),
        "education" => Ok(Education::delete(conn, id)),
        "kind" => Ok(Kind::delete(conn, id)),
        "post" => Ok(Post::delete(conn, id)),
        "practice" => Ok(Practice::delete(conn, id)),
        "rank" => Ok(Rank::delete(conn, id)),
        "scope" => Ok(Scope::delete(conn, id)),
        "siren" => Ok(Siren::delete(conn, id)),
        "siren_type" => Ok(SirenType::delete(conn, id)),
        _ => Err(format!("bad path {}", name)),
    }
}

fn get_children(conn: &Connection, name: &str, children: &str, id: i64) -> Result<DBList, String> {
    match (name, children) {
        ("company", "practice") => Ok(DBList::PracticeList(PracticeList::get_by_company(
            conn, id,
        )?)),
        _ => Err("bad path".to_string()),
    }
}

fn http_result_list(res: Result<DBList, BlockingError<std::string::String>>) -> HttpResponse {
    match res {
        Ok(db_result) => HttpResponse::Ok().json(json!({
            "data": db_result,
            "error": Null,
            "ok": true
        })),
        Err(err) => HttpResponse::Ok().json(json!({
            "data": Null,
            "error": err.to_string(),
            "ok": false
        })),
    }
}

fn http_result_item(res: Result<DBItem, BlockingError<std::string::String>>) -> HttpResponse {
    match res {
        Ok(db_result) => HttpResponse::Ok().json(json!({
            "data": db_result,
            "error": Null,
            "ok": true
        })),
        Err(err) => HttpResponse::Ok().json(json!({
            "data": Null,
            "error": err.to_string(),
            "ok": false
        })),
    }
}

pub fn get_name_children(
    id: Identity,
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, String, i64)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let a = check_auth(id);
    web::block(move || {
        // a?;
        let conn = db.get().unwrap();
        get_children(&conn, &path.0, &path.1, path.2)
    })
    .then(|res| Ok(http_result_list(res)))
}

pub fn get_name_command(
    id: Identity,
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let a = check_auth(id);
    web::block(move || {
        // a?;
        let conn = db.get().unwrap();
        get_list(&conn, &path.0, &path.1)
    })
    .then(|res| Ok(http_result_list(res)))
}

pub fn get_name_id(
    id: Identity,
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, i64)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let a = check_auth(id);
    web::block(move || {
        // a?;
        let conn = db.get().unwrap();
        get_item(&conn, &path.0, path.1)
    })
    .then(|res| Ok(http_result_item(res)))
}

pub fn post_name_id(
    id: Identity,
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, i64)>,
    params: web::Json<DBItem>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let a = check_auth(id);
    web::block(move || {
        // a?;
        let conn = db.get().unwrap();
        post_item(&conn, &path.0, path.1, params)
    })
    .then(|res| Ok(http_result_item(res)))
}

pub fn delete_name_id(
    id: Identity,
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(String, i64)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let a = check_auth(id);
    web::block(move || {
        a?;
        let conn = db.get().unwrap();
        delete_item(&conn, &path.0, path.1)
    })
    .then(|res| match res {
        Ok(res) => HttpResponse::Ok().json(json!({
            "data": Null,
            "error": Null,
            "ok": res
        })),
        Err(err) => HttpResponse::Ok().json(json!({
            "data": Null,
            "error": err.to_string(),
            "ok": false
        })),
    })
}

// pub fn test_post_name_id(
//     id: Identity,
//     _db: web::Data<Pool<PostgresConnectionManager>>,
//     path: web::Path<(String, i64)>,
//     params: web::Json<TestStruct>,
// ) -> HttpResponse {
//     let a = check_auth(id);
//     let values = params.into_inner();
//     println!("{} {} {:?} {:?}", path.0, path.1, a, values);
//     HttpResponse::Ok().json(json!({
//         "data": values,
//         "error": Null,
//         "ok": true
//     }))
// }
