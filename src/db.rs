use std::fmt;

use actix_web::{error::BlockingError, web, HttpResponse};
use anyhow::{anyhow, Result};
use deadpool_postgres::{Pool, Client};
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

#[derive(Serialize)]
pub struct Msg {
    pub name: String,
    pub object: DBObject,
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub id: i64,
}

#[derive(Deserialize)]
pub enum Object {
    Item(Item),
    List(String),
}

#[derive(Deserialize)]
pub enum Command {
    Get(Object),
    Set(DBObject),
}

#[derive(Deserialize, Serialize)]
pub enum DBObject {
    Null,
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

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Item(i) => write!(f, "Item {} {}", i.id, i.name),
            Object::List(s) => write!(f, "List {}", s),
        }
    }
}

// pub async fn get_object(object: &Object, pool: Pool) -> Msg {
//     match object {
//         Object::Item(item) => match get_item(item, pool).await {
//             Ok(db_object) => Msg {
//                 name: item.name.clone(),
//                 object: db_object,
//                 error: String::new(),
//             },
//             Err(err) => Msg {
//                 name: item.name.clone(),
//                 object: DBObject::Null,
//                 error: err.to_string(),
//             },
//         },
//         Object::List(obj) => match get_list(obj, pool).await {
//             Ok(db_object) => Msg {
//                 name: obj.clone(),
//                 object: db_object,
//                 error: String::new(),
//             },
//             Err(err) => Msg {
//                 name: obj.clone(),
//                 object: DBObject::Null,
//                 error: err.to_string(),
//             },
//         },
//     }
// }

async fn get_item(item: &Item, client: &Client) -> Result<DBObject> {
    match (item.name.as_str(), item.id) {
        ("Certificate", id) => Ok(DBObject::Certificate(Certificate::get(&client, id).await?)),
        ("Company", id) => Ok(DBObject::Company(Box::new(
            Company::get(&client, id).await?,
        ))),
        ("Contact", id) => Ok(DBObject::Contact(Box::new(
            Contact::get(&client, id).await?,
        ))),
        ("Department", id) => Ok(DBObject::Department(Department::get(&client, id).await?)),
        ("Education", id) => Ok(DBObject::Education(Education::get(&client, id).await?)),
        ("Kind", id) => Ok(DBObject::Kind(Kind::get(&client, id).await?)),
        ("Post", id) => Ok(DBObject::Post(Post::get(&client, id).await?)),
        ("Practice", id) => Ok(DBObject::Practice(Practice::get(&client, id).await?)),
        ("Rank", id) => Ok(DBObject::Rank(Rank::get(&client, id).await?)),
        ("Scope", id) => Ok(DBObject::Scope(Scope::get(&client, id).await?)),
        ("Siren", id) => Ok(DBObject::Siren(Box::new(Siren::get(&client, id).await?))),
        ("SirenType", id) => Ok(DBObject::SirenType(SirenType::get(&client, id).await?)),
        (e, id) => Err(anyhow!("bad item object: {} {}", e, id)),
    }
}

async fn get_list(object: &String, pool: Pool) -> Result<DBObject> {
    let client = pool.get().await?;
    match object.as_str() {
        "CertificateList" => Ok(DBObject::CertificateList(
            CertificateList::get_all(&client).await?,
        )),
        "CompanyList" => Ok(DBObject::CompanyList(CompanyList::get_all(&client).await?)),
        "CompanySelect" => Ok(DBObject::SelectItem(
            SelectItem::company_all(&client).await?,
        )),
        "ContactList" => Ok(DBObject::ContactList(ContactList::get_all(&client).await?)),
        "ContactSelect" => Ok(DBObject::SelectItem(
            SelectItem::contact_all(&client).await?,
        )),
        "DepartmentList" => Ok(DBObject::DepartmentList(
            DepartmentList::get_all(&client).await?,
        )),
        "DepartmentSelect" => Ok(DBObject::SelectItem(
            SelectItem::department_all(&client).await?,
        )),
        "EducationList" => Ok(DBObject::EducationList(
            EducationList::get_all(&client).await?,
        )),
        "EducationNear" => Ok(DBObject::EducationShort(
            EducationShort::get_near(&client).await?,
        )),
        // "EducationShort" =>
        "KindList" => Ok(DBObject::KindList(KindList::get_all(&client).await?)),
        "KindSelect" => Ok(DBObject::SelectItem(SelectItem::kind_all(&client).await?)),
        "PostList" => Ok(DBObject::PostList(PostList::get_all(&client).await?)),
        "PostSelect" => Ok(DBObject::SelectItem(
            SelectItem::post_all(&client, false).await?,
        )),
        "PostGoSelect" => Ok(DBObject::SelectItem(
            SelectItem::post_all(&client, true).await?,
        )),
        "PracticeList" => Ok(DBObject::PracticeList(
            PracticeList::get_all(&client).await?,
        )),
        "PracticeNear" => Ok(DBObject::PracticeShort(
            PracticeShort::get_near(&client).await?,
        )),
        // "PracticeShort" =>
        "RankList" => Ok(DBObject::RankList(RankList::get_all(&client).await?)),
        "RankSelect" => Ok(DBObject::SelectItem(SelectItem::rank_all(&client).await?)),
        "ScopeList" => Ok(DBObject::ScopeList(ScopeList::get_all(&client).await?)),
        "ScopeSelect" => Ok(DBObject::SelectItem(SelectItem::scope_all(&client).await?)),
        // "SelectItem" =>
        "SirenList" => Ok(DBObject::SirenList(SirenList::get_all(&client).await?)),
        "SirenTypeList" => Ok(DBObject::SirenTypeList(
            SirenTypeList::get_all(&client).await?,
        )),
        "SirenTypeSelect" => Ok(DBObject::SelectItem(
            SelectItem::siren_type_all(&client).await?,
        )),
        e => Err(anyhow!("bad list object: {}", e)),
    }
}

async fn post_item(
    client: &Client,
    name: &str,
    id: i64,
    params: web::Json<DBObject>,
) -> Result<DBObject> {
    match (name, params.into_inner()) {
        ("certificate", DBObject::Certificate(item)) => Ok(DBObject::Certificate(
            Certificate::insert(client, item).await?,
        )),
        ("company", DBObject::Company(item)) => Ok(DBObject::Company(Box::new(
            Company::insert(client, *item).await?,
        ))),
        ("contact", DBObject::Contact(item)) => Ok(DBObject::Contact(Box::new(
            Contact::insert(client, *item).await?,
        ))),
        ("department", DBObject::Department(item)) => Ok(DBObject::Department(
            Department::insert(client, item).await?,
        )),
        ("education", DBObject::Education(item)) => Ok(DBObject::Education(
            Education::insert(client, item).await?,
        )),
        ("kind", DBObject::Kind(item)) => Ok(DBObject::Kind(Kind::insert(client, item).await?)),
        ("post", DBObject::Post(item)) => Ok(DBObject::Post(Post::insert(client, item).await?)),
        ("practice", DBObject::Practice(item)) => {
            Ok(DBObject::Practice(Practice::insert(client, item).await?))
        }
        ("rank", DBObject::Rank(item)) => Ok(DBObject::Rank(Rank::insert(client, item).await?)),
        ("scope", DBObject::Scope(item)) => Ok(DBObject::Scope(Scope::insert(client, item).await?)),
        ("siren", DBObject::Siren(item)) => {
            Ok(DBObject::Siren(Box::new(Siren::insert(client, *item).await?)))
        }
        ("siren_type", DBObject::SirenType(item)) => {
            Ok(DBObject::SirenType(SirenType::insert(client, item).await?))
        }
        _ => Err(anyhow!("bad path {}", name)),
    }
}

async fn delete_item(client: &Client, name: &str, id: i64) -> Result<u64> {
    match name {
        "certificate" => Ok(Certificate::delete(client, id).await?),
        "company" => Ok(Company::delete(client, id).await?),
        "contact" => Ok(Contact::delete(client, id).await?),
        "department" => Ok(Department::delete(client, id).await?),
        "education" => Ok(Education::delete(client, id).await?),
        "kind" => Ok(Kind::delete(client, id).await?),
        "post" => Ok(Post::delete(client, id).await?),
        "practice" => Ok(Practice::delete(client, id).await?),
        "rank" => Ok(Rank::delete(client, id).await?),
        "scope" => Ok(Scope::delete(client, id).await?),
        "siren" => Ok(Siren::delete(client, id).await?),
        "siren_type" => Ok(SirenType::delete(client, id).await?),
        _ => Err(anyhow!("bad path {:?}", name)),
    }
}

// async fn get_children(
//     client: &Client,
//     name: &str,
//     children: &str,
//     id: i64,
// ) -> Result<DBObject, Error> {
//     match (name, children) {
//         ("company", "practice") => Ok(DBObject::PracticeList(
//             PracticeList::get_by_company(client, id).await?,
//         )),
//         _ => Err(anyhow!("bad path")),
//     }
// }

// fn http_result_list(res: Result<DBObject, Error>) -> HttpResponse {
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

fn http_result_item(res: Result<DBObject>) -> HttpResponse {
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

pub async fn get_name_list_children_id(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, String, i64)>,
) -> Result<HttpResponse> {
    // let a = check_auth(id);
    // a?;
    let client = db.get().await?;
    let res = get_children(&client, &path.0, &path.1, path.2).await;
    Ok(http_result_list(res))
}

// pub async fn get_name_command(
//     // id: Identity,
//     db: web::Data<Pool>,
//     path: web::Path<(String, String)>,
// ) -> Result<HttpResponse, Error> {
//     // let a = check_auth(id);
//     //         a?;
//     let client = db.get().await?;
//     let res = get_list(&client, &path.0, &path.1).await;
//     Ok(http_result_list(res))
// }

pub async fn get_name_id(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, i64)>,
) -> Result<HttpResponse> {
    // let a = check_auth(id);
    // a?;
    let client = db.get().await?;
    let res = get_item(&Item{name: path.0, id: path.1}, &client).await;
    Ok(http_result_item(res))
}

pub async fn post_name_id(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, i64)>,
    params: web::Json<DBObject>,
) -> Result<HttpResponse> {
    // let a = check_auth(id);
    // a?;
    let client = db.get().await?;
    let res = post_item(&client, &path.0, path.1, params);
    Ok(http_result_item(res))
}

pub async fn delete_name_id(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, i64)>,
) -> Result<HttpResponse> {
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

