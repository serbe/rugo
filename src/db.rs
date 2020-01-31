// use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use anyhow::{anyhow, Error};
use deadpool_postgres::{Client, Pool};
use dotenv::dotenv;
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

// use crate::auth::check_auth;

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

async fn get_list(client: &Client, name: &str, command: &str) -> Result<DBList, Error> {
    match (name, command) {
        ("certificate", "list") => Ok(DBList::CertificateList(
            CertificateList::get_all(client).await?,
        )),
        ("company", "list") => Ok(DBList::CompanyList(CompanyList::get_all(client).await?)),
        ("company", "select") => Ok(DBList::SelectItem(SelectItem::company_all(client).await?)),
        ("contact", "list") => Ok(DBList::ContactList(ContactList::get_all(client).await?)),
        ("contact", "select") => Ok(DBList::SelectItem(SelectItem::contact_all(client).await?)),
        ("department", "list") => Ok(DBList::DepartmentList(
            DepartmentList::get_all(client).await?,
        )),
        ("department", "select") => Ok(DBList::SelectItem(
            SelectItem::department_all(client).await?,
        )),
        ("education", "list") => Ok(DBList::EducationList(EducationList::get_all(client).await?)),
        ("education", "near") => Ok(DBList::EducationShort(
            EducationShort::get_near(client).await?,
        )),
        ("kind", "list") => Ok(DBList::KindList(KindList::get_all(client).await?)),
        ("kind", "select") => Ok(DBList::SelectItem(SelectItem::kind_all(client).await?)),
        ("post", "list") => Ok(DBList::PostList(PostList::get_all(client).await?)),
        ("post", "select") => Ok(DBList::SelectItem(
            SelectItem::post_all(client, false).await?,
        )),
        ("post_go", "select") => Ok(DBList::SelectItem(
            SelectItem::post_all(client, true).await?,
        )),
        ("practice", "list") => Ok(DBList::PracticeList(PracticeList::get_all(client).await?)),
        ("practice", "near") => Ok(DBList::PracticeShort(
            PracticeShort::get_near(client).await?,
        )),
        ("rank", "list") => Ok(DBList::RankList(RankList::get_all(client).await?)),
        ("rank", "select") => Ok(DBList::SelectItem(SelectItem::rank_all(client).await?)),
        ("scope", "list") => Ok(DBList::ScopeList(ScopeList::get_all(client).await?)),
        ("scope", "select") => Ok(DBList::SelectItem(SelectItem::scope_all(client).await?)),
        ("siren", "list") => Ok(DBList::SirenList(SirenList::get_all(client).await?)),
        ("siren_type", "list") => Ok(DBList::SirenTypeList(SirenTypeList::get_all(client).await?)),
        ("siren_type", "select") => Ok(DBList::SelectItem(
            SelectItem::siren_type_all(client).await?,
        )),
        _ => Err(anyhow!("bad path")),
    }
}

async fn get_item(client: &Client, name: &str, id: i64) -> Result<DBItem, Error> {
    match name {
        "certificate" => Ok(DBItem::Certificate(Certificate::get(client, id).await?)),
        "company" => Ok(DBItem::Company(Box::new(Company::get(client, id).await?))),
        "contact" => Ok(DBItem::Contact(Box::new(Contact::get(client, id).await?))),
        "department" => Ok(DBItem::Department(Department::get(client, id).await?)),
        "education" => Ok(DBItem::Education(Education::get(client, id).await?)),
        "kind" => Ok(DBItem::Kind(Kind::get(client, id).await?)),
        "post" => Ok(DBItem::Post(Post::get(client, id).await?)),
        "practice" => Ok(DBItem::Practice(Practice::get(client, id).await?)),
        "rank" => Ok(DBItem::Rank(Rank::get(client, id).await?)),
        "scope" => Ok(DBItem::Scope(Scope::get(client, id).await?)),
        "siren" => Ok(DBItem::Siren(Box::new(Siren::get(client, id).await?))),
        "siren_type" => Ok(DBItem::SirenType(SirenType::get(client, id).await?)),
        _ => Err(anyhow!("bad path")),
    }
}

// async fn post_item(
//     client: &Client,
//     name: &str,
//     id: i64,
//     params: web::Json<DBItem>,
// ) -> Result<DBItem, Error> {
//     match (name, params.into_inner()) {
//         ("certificate", DBItem::Certificate(item)) => {
//             Ok(DBItem::Certificate(Certificate::post(client, id, item).await?))
//         }
//         ("company", DBItem::Company(item)) => {
//             Ok(DBItem::Company(Box::new(Company::post(client, id, *item).await?)))
//         }
//         ("contact", DBItem::Contact(item)) => {
//             Ok(DBItem::Contact(Box::new(Contact::post(client, id, *item).await?)))
//         }
//         ("department", DBItem::Department(item)) => {
//             Ok(DBItem::Department(Department::post(client, id, item).await?))
//         }
//         ("education", DBItem::Education(item)) => {
//             Ok(DBItem::Education(Education::post(client, id, item).await?))
//         }
//         ("kind", DBItem::Kind(item)) => Ok(DBItem::Kind(Kind::post(client, id, item).await?)),
//         ("post", DBItem::Post(item)) => Ok(DBItem::Post(Post::post(client, id, item).await?)),
//         ("practice", DBItem::Practice(item)) => {
//             Ok(DBItem::Practice(Practice::post(client, id, item)?))
//         }
//         ("rank", DBItem::Rank(item)) => Ok(DBItem::Rank(Rank::post(client, id, item)?)),
//         ("scope", DBItem::Scope(item)) => Ok(DBItem::Scope(Scope::post(client, id, item)?)),
//         ("siren", DBItem::Siren(item)) => {
//             Ok(DBItem::Siren(Box::new(Siren::post(client, id, *item)?)))
//         }
//         ("siren_type", DBItem::SirenType(item)) => {
//             Ok(DBItem::SirenType(SirenType::post(client, id, item)?))
//         }
//         _ => Err(format!("bad path {}", name)),
//     }
// }

fn delete_item(client: &Client, name: &str, id: i64) -> Result<bool, Error> {
    match name {
        "certificate" => Ok(Certificate::delete(client, id)),
        "company" => Ok(Company::delete(client, id)),
        "contact" => Ok(Contact::delete(client, id)),
        "department" => Ok(Department::delete(client, id)),
        "education" => Ok(Education::delete(client, id)),
        "kind" => Ok(Kind::delete(client, id)),
        "post" => Ok(Post::delete(client, id)),
        "practice" => Ok(Practice::delete(client, id)),
        "rank" => Ok(Rank::delete(client, id)),
        "scope" => Ok(Scope::delete(client, id)),
        "siren" => Ok(Siren::delete(client, id)),
        "siren_type" => Ok(SirenType::delete(client, id)),
        _ => Err(anyhow!("bad path {:?}", name)),
    }
}

async fn get_children(
    client: &Client,
    name: &str,
    children: &str,
    id: i64,
) -> Result<DBList, Error> {
    match (name, children) {
        ("company", "practice") => Ok(DBList::PracticeList(
            PracticeList::get_by_company(client, id).await?,
        )),
        _ => Err(anyhow!("bad path")),
    }
}

fn http_result_list(res: Result<DBList, Error>) -> HttpResponse {
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

fn http_result_item(res: Result<DBItem, Error>) -> HttpResponse {
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

pub async fn get_name_children(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, String, i64)>,
) -> Result<HttpResponse, Error> {
    // let a = check_auth(id);
    // a?;
    let client = db.get().await?;
    let res = get_children(&client, &path.0, &path.1, path.2).await;
    Ok(http_result_list(res))
}

pub async fn get_name_command(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    // let a = check_auth(id);
    //         a?;
    let client = db.get().await?;
    let res = get_list(&client, &path.0, &path.1).await;
    Ok(http_result_list(res))
}

pub async fn get_name_id(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, i64)>,
) -> Result<HttpResponse, Error> {
    // let a = check_auth(id);
    // a?;
    let client = db.get().await?;
    let res = get_item(&client, &path.0, path.1).await;
    Ok(http_result_item(res))
}

// pub async fn post_name_id(
//     id: Identity,
//     db: web::Data<Pool>,
//     path: web::Path<(String, i64)>,
//     params: web::Json<DBItem>,
// ) -> Result<HttpResponse, Error> {
//     let a = check_auth(id);
//         a?;
//         let client = db.get().await?;
//         let res = post_item(&client, &path.0, path.1, params);
//     Ok(http_result_item(res))
// }

pub async fn delete_name_id(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, i64)>,
) -> Result<HttpResponse, Error> {
    // let a = check_auth(id);
    //     a?;
    let client = db.get().await?;
    let res = delete_item(&client, &path.0, path.1);
    Ok(match res {
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
