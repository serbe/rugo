// use actix_identity::Identity;
// use actix_web::{error::BlockingError, web, Error, HttpResponse};
// use futures::Future;
use postgres::{Client, NoTls};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// use crate::auth::check_auth;
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

#[derive(Serialize)]
pub struct DBResult {
    data: Value,
}

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

// #[derive(Serialize)]
// pub enum DBList {
//     CertificateList(Vec<CertificateList>),
//     CompanyList(Vec<CompanyList>),
//     ContactList(Vec<ContactList>),
//     DepartmentList(Vec<DepartmentList>),
//     EducationList(Vec<EducationList>),
//     KindList(Vec<KindList>),
//     PostList(Vec<PostList>),
//     PracticeList(Vec<PracticeList>),
//     RankList(Vec<RankList>),
//     ScopeList(Vec<ScopeList>),
//     SelectItem(Vec<SelectItem>),
//     SirenList(Vec<SirenList>),
//     SirenTypeList(Vec<SirenTypeList>),
// }

// #[derive(Serialize)]
// pub enum DBShort {
//     EducationShort(Vec<EducationShort>),
//     PracticeShort(Vec<PracticeShort>),
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct TestStruct {
//     value: Value,
// }

fn get_connurl() -> String {
    let dbname = dotenv::var("DB_NAME").expect("missing env DB_NAME");
    let dbuser = dotenv::var("DB_USER");
    let dbpassword = dotenv::var("DB_PASSWORD");
    let dbhost = dotenv::var("DB_HOST");
    let dbport = dotenv::var("DB_PORT");
    let mut cfgstr = format!("dbname={}", dbname);
    if let Ok(user) = dbuser {
        cfgstr.push_str(format!(" user={}", user).as_str())
    };
    if let Ok(password) = dbpassword {
        cfgstr.push_str(format!(" password={}", password).as_str())
    };
    if let Ok(host) = dbhost {
        cfgstr.push_str(format!(" host={}", host).as_str())
    };
    if let Ok(port) = dbport {
        cfgstr.push_str(format!(" port={}", port).as_str())
    };
    cfgstr
}

pub fn get_manager() -> PostgresConnectionManager<NoTls> {
    let conn_url = get_connurl();
    PostgresConnectionManager::new(conn_url.parse().unwrap(), NoTls)
}

pub fn get_pool() -> Pool<PostgresConnectionManager<NoTls>> {
    r2d2::Pool::new(get_manager()).expect("error create r2d2 pool")
}

pub fn get_item(conn: &mut Client, id: i64, name: String) -> Result<Value, String> {
    match name.as_str() {
        "certificate" => Ok(json!(Certificate::get(conn, id)?)),
        "company" => Ok(json!(Company::get(conn, id)?)),
        "contact" => Ok(json!(Contact::get(conn, id)?)),
        "department" => Ok(json!(Department::get(conn, id)?)),
        "education" => Ok(json!(Education::get(conn, id)?)),
        "kind" => Ok(json!(Kind::get(conn, id)?)),
        "post" => Ok(json!(Post::get(conn, id)?)),
        "practice" => Ok(json!(Practice::get(conn, id)?)),
        "rank" => Ok(json!(Rank::get(conn, id)?)),
        "scope" => Ok(json!(Scope::get(conn, id)?)),
        "siren" => Ok(json!(Siren::get(conn, id)?)),
        "siren_type" => Ok(json!(SirenType::get(conn, id)?)),
        _ => Err("bad path".to_string()),
    }
}

pub fn get_list(conn: &mut Client, name: String) -> Result<Value, String> {
    match name.as_str() {
        "certificate" => Ok(json!(CertificateList::get_all(conn)?)),
        "company" => Ok(json!(CompanyList::get_all(conn)?)),
        "contact" => Ok(json!(ContactList::get_all(conn)?)),
        "department" => Ok(json!(DepartmentList::get_all(conn)?)),
        "education" => Ok(json!(EducationList::get_all(conn)?)),
        "kind" => Ok(json!(KindList::get_all(conn)?)),
        "post" => Ok(json!(PostList::get_all(conn)?)),
        "practice" => Ok(json!(PracticeList::get_all(conn)?)),
        "rank" => Ok(json!(RankList::get_all(conn)?)),
        "scope" => Ok(json!(ScopeList::get_all(conn)?)),
        "siren" => Ok(json!(SirenList::get_all(conn)?)),
        "siren_type" => Ok(json!(SirenTypeList::get_all(conn)?)),
        _ => Err("bad path".to_string()),
    }
}

pub fn get_near(conn: &mut Client, name: String) -> Result<Value, String> {
    match name.as_str() {
        "education" => Ok(json!(EducationShort::get_near(conn)?)),
        "practice" => Ok(json!(PracticeShort::get_near(conn)?)),
        _ => Err("bad path".to_string()),
    }
}

pub fn get_select(conn: &mut Client, name: String) -> Result<Value, String> {
    match name.as_str() {
        "company" => Ok(json!(SelectItem::company_all(conn)?)),
        "contact" => Ok(json!(SelectItem::contact_all(conn)?)),
        "department" => Ok(json!(SelectItem::department_all(conn)?)),
        "kind" => Ok(json!(SelectItem::kind_all(conn)?)),
        "post" => Ok(json!(SelectItem::post_all(conn, false)?)),
        "post_go" => Ok(json!(SelectItem::post_all(conn, true)?)),
        "rank" => Ok(json!(SelectItem::rank_all(conn)?)),
        "scope" => Ok(json!(SelectItem::scope_all(conn)?)),
        "siren_type" => Ok(json!(SelectItem::siren_type_all(conn)?)),
        _ => Err("bad path".to_string()),
    }
}

pub fn insert_item(conn: &mut Client, item: DBItem) -> Result<Value, String> {
    match item {
        DBItem::Certificate(item) => Ok(json!(Certificate::insert(conn, item)?)),
        DBItem::Company(item) => Ok(json!(Company::insert(conn, *item)?)),
        DBItem::Contact(item) => Ok(json!(Contact::insert(conn, *item)?)),
        DBItem::Department(item) => Ok(json!(Department::insert(conn, item)?)),
        DBItem::Education(item) => Ok(json!(Education::insert(conn, item)?)),
        DBItem::Kind(item) => Ok(json!(Kind::insert(conn, item)?)),
        DBItem::Post(item) => Ok(json!(Post::insert(conn, item)?)),
        DBItem::Practice(item) => Ok(json!(Practice::insert(conn, item)?)),
        DBItem::Rank(item) => Ok(json!(Rank::insert(conn, item)?)),
        DBItem::Scope(item) => Ok(json!(Scope::insert(conn, item)?)),
        DBItem::Siren(item) => Ok(json!(Siren::insert(conn, *item)?)),
        DBItem::SirenType(item) => Ok(json!(SirenType::insert(conn, item)?)),
    }
}

pub fn update_item(conn: &mut Client, item: DBItem) -> Result<Value, String> {
    match item {
        DBItem::Certificate(item) => Ok(json!(Certificate::update(conn, item)?)),
        DBItem::Company(item) => Ok(json!(Company::update(conn, *item)?)),
        DBItem::Contact(item) => Ok(json!(Contact::update(conn, *item)?)),
        DBItem::Department(item) => Ok(json!(Department::update(conn, item)?)),
        DBItem::Education(item) => Ok(json!(Education::update(conn, item)?)),
        DBItem::Kind(item) => Ok(json!(Kind::update(conn, item)?)),
        DBItem::Post(item) => Ok(json!(Post::update(conn, item)?)),
        DBItem::Practice(item) => Ok(json!(Practice::update(conn, item)?)),
        DBItem::Rank(item) => Ok(json!(Rank::update(conn, item)?)),
        DBItem::Scope(item) => Ok(json!(Scope::update(conn, item)?)),
        DBItem::Siren(item) => Ok(json!(Siren::update(conn, *item)?)),
        DBItem::SirenType(item) => Ok(json!(SirenType::update(conn, item)?)),
    }
}

pub fn delete_item(conn: &mut Client, id: i64, name: String) -> Result<Value, String> {
    match name.as_str() {
        "certificate" => Ok(json!(Certificate::delete(conn, id))),
        "company" => Ok(json!(Company::delete(conn, id))),
        "contact" => Ok(json!(Contact::delete(conn, id))),
        "department" => Ok(json!(Department::delete(conn, id))),
        "education" => Ok(json!(Education::delete(conn, id))),
        "kind" => Ok(json!(Kind::delete(conn, id))),
        "post" => Ok(json!(Post::delete(conn, id))),
        "practice" => Ok(json!(Practice::delete(conn, id))),
        "rank" => Ok(json!(Rank::delete(conn, id))),
        "scope" => Ok(json!(Scope::delete(conn, id))),
        "siren" => Ok(json!(Siren::delete(conn, id))),
        "siren_type" => Ok(json!(SirenType::delete(conn, id))),
        _ => Err(format!("bad path {}", name)),
    }
}

// fn get_children(conn: &mut Client, name: &str, children: &str, id: i64) -> Result<DBList, String> {
//     match (name, children) {
//         ("company", "practice") => Ok(DBList::PracticeList(PracticeList::get_by_company(
//             conn, id,
//         )?)),
//         _ => Err("bad path".to_string()),
//     }
// }

// fn http_result_list(res: Result<DBList, BlockingError<std::string::String>>) -> HttpResponse {
//     match res {
//         Ok(db_result) => HttpResponse::Ok().json(json!({
//             "data": db_result,
//             "error": Null,
//             "ok": true
//         })),
//         Err(err) => HttpResponse::Ok().json(json!({
//             "data": Null,
//             "error": err.to_string(),
//             "ok": false
//         })),
//     }
// }

// fn http_result_item(res: Result<DBItem, BlockingError<std::string::String>>) -> HttpResponse {
//     match res {
//         Ok(db_result) => HttpResponse::Ok().json(json!({
//             "data": db_result,
//             "error": Null,
//             "ok": true
//         })),
//         Err(err) => HttpResponse::Ok().json(json!({
//             "data": Null,
//             "error": err.to_string(),
//             "ok": false
//         })),
//     }
// }

// pub fn get_name_children(
//     // id: Identity,
//     db: web::Data<Pool<PostgresConnectionManager>>,
//     path: web::Path<(String, String, i64)>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     // let a = check_auth(id);
//     web::block(move || {
//         // a?;
//         let conn = db.get().unwrap();
//         get_children(&conn, &path.0, &path.1, path.2)
//     })
//     .then(|res| Ok(http_result_list(res)))
// }

// pub fn get_name_command(
//     // id: Identity,
//     db: web::Data<Pool<PostgresConnectionManager>>,
//     path: web::Path<(String, String)>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     // let a = check_auth(id);
//     web::block(move || {
//         // a?;
//         let conn = db.get().unwrap();
//         get_list(&conn, &path.0, &path.1)
//     })
//     .then(|res| Ok(http_result_list(res)))
// }

// pub fn get_name_id(
//     // id: Identity,
//     db: web::Data<Pool<PostgresConnectionManager>>,
//     path: web::Path<(String, i64)>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     // let a = check_auth(id);
//     web::block(move || {
//         // a?;
//         let conn = db.get().unwrap();
//         get_item(&conn, &path.0, path.1)
//     })
//     .then(|res| Ok(http_result_item(res)))
// }

// pub fn post_name_id(
//     // id: Identity,
//     db: web::Data<Pool<PostgresConnectionManager>>,
//     path: web::Path<(String, i64)>,
//     params: web::Json<DBItem>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     // let a = check_auth(id);
//     web::block(move || {
//         // a?;
//         let conn = db.get().unwrap();
//         post_item(&conn, &path.0, path.1, params)
//     })
//     .then(|res| Ok(http_result_item(res)))
// }

// pub fn delete_name_id(
//     id: Identity,
//     db: web::Data<Pool<PostgresConnectionManager>>,
//     path: web::Path<(String, i64)>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     let a = check_auth(id);
//     web::block(move || {
//         a?;
//         let conn = db.get().unwrap();
//         delete_item(&conn, &path.0, path.1)
//     })
//     .then(|res| match res {
//         Ok(res) => HttpResponse::Ok().json(json!({
//             "data": Null,
//             "error": Null,
//             "ok": res
//         })),
//         Err(err) => HttpResponse::Ok().json(json!({
//             "data": Null,
//             "error": err.to_string(),
//             "ok": false
//         })),
//     })
// }

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
