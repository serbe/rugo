use std::fmt;

use actix_web::{web, HttpResponse};
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value::Null};

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

use crate::error::ServiceError;

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

impl Item {
    pub fn from(path: (String, i64)) -> Self {
        Item {
            name: path.0.clone(),
            id: path.1,
        }
    }
}

#[derive(Deserialize)]
pub enum Object {
    Item(Item),
    List(String),
}

#[derive(Deserialize)]
pub enum Command {
    Get(Object),
    Insert(DBObject),
    Update(DBObject),
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

pub async fn jsonpost(
    db: web::Data<Pool>,
    params: web::Json<Command>,
) -> Result<HttpResponse, ServiceError> {
    let cmd: Command = params.into_inner();
    match cmd {
        Command::Get(object) => get_object(&object, &db.get().await?).await,
        Command::Insert(dbobject) => insert_object(dbobject, &db.get().await?).await,
        Command::Update(dbobject) => update_object(dbobject, &db.get().await?).await,
    }
}

pub async fn get_object(object: &Object, client: &Client) -> Result<HttpResponse, ServiceError> {
    let msg = match object {
        Object::Item(item) => match get_item(item, client).await {
            Ok(db_object) => Msg {
                name: item.name.clone(),
                object: db_object,
                error: String::new(),
            },
            Err(err) => Msg {
                name: item.name.clone(),
                object: DBObject::Null,
                error: err.to_string(),
            },
        },
        Object::List(obj) => match get_list(obj, client).await {
            Ok(db_object) => Msg {
                name: obj.clone(),
                object: db_object,
                error: String::new(),
            },
            Err(err) => Msg {
                name: obj.clone(),
                object: DBObject::Null,
                error: err.to_string(),
            },
        },
    };
    Ok(HttpResponse::Ok().json(msg))
}

pub async fn insert_object(
    dbobject: DBObject,
    client: &Client,
) -> Result<HttpResponse, ServiceError> {
    let msg = match insert_item(dbobject, &client).await {
        Ok(db_object) => Msg {
            name: String::new(),
            object: db_object,
            error: String::new(),
        },
        Err(err) => Msg {
            name: String::new(),
            object: DBObject::Null,
            error: err.to_string(),
        },
    };
    Ok(HttpResponse::Ok().json(msg))
}

pub async fn update_object(
    dbobject: DBObject,
    client: &Client,
) -> Result<HttpResponse, ServiceError> {
    let msg = match update_item(dbobject, &client).await {
        Ok(_) => Msg {
            name: String::new(),
            object: DBObject::Null,
            error: String::new(),
        },
        Err(err) => Msg {
            name: String::new(),
            object: DBObject::Null,
            error: err.to_string(),
        },
    };
    Ok(HttpResponse::Ok().json(msg))
}

async fn get_item(item: &Item, client: &Client) -> Result<DBObject, ServiceError> {
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
        (e, id) => Err(ServiceError::BadRequest(format!(
            "bad item object: {} {}",
            e, id
        ))),
    }
}

async fn get_list(name: &String, client: &Client) -> Result<DBObject, ServiceError> {
    match name.as_str() {
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
        e => Err(ServiceError::BadRequest(format!("bad list object: {}", e))),
    }
}

async fn insert_item(object: DBObject, client: &Client) -> Result<DBObject, ServiceError> {
    match object {
        DBObject::Certificate(item) => Ok(DBObject::Certificate(
            Certificate::insert(&client, item).await?,
        )),
        DBObject::Company(item) => Ok(DBObject::Company(Box::new(
            Company::insert(&client, *item).await?,
        ))),
        DBObject::Contact(item) => Ok(DBObject::Contact(Box::new(
            Contact::insert(&client, *item).await?,
        ))),
        DBObject::Department(item) => Ok(DBObject::Department(
            Department::insert(&client, item).await?,
        )),
        DBObject::Education(item) => {
            Ok(DBObject::Education(Education::insert(&client, item).await?))
        }
        DBObject::Kind(item) => Ok(DBObject::Kind(Kind::insert(&client, item).await?)),
        DBObject::Post(item) => Ok(DBObject::Post(Post::insert(&client, item).await?)),
        DBObject::Practice(item) => Ok(DBObject::Practice(Practice::insert(&client, item).await?)),
        DBObject::Rank(item) => Ok(DBObject::Rank(Rank::insert(&client, item).await?)),
        DBObject::Scope(item) => Ok(DBObject::Scope(Scope::insert(&client, item).await?)),
        DBObject::Siren(item) => Ok(DBObject::Siren(Box::new(
            Siren::insert(&client, *item).await?,
        ))),
        DBObject::SirenType(item) => {
            Ok(DBObject::SirenType(SirenType::insert(&client, item).await?))
        }
        _ => Err(ServiceError::BadRequest("bad item object".to_string())),
    }
}

async fn update_item(object: DBObject, client: &Client) -> Result<u64, ServiceError> {
    match object {
        DBObject::Certificate(item) => Ok(Certificate::update(&client, item).await?),
        DBObject::Company(item) => Ok(Company::update(&client, *item).await?),
        DBObject::Contact(item) => Ok(Contact::update(&client, *item).await?),
        DBObject::Department(item) => Ok(Department::update(&client, item).await?),
        DBObject::Education(item) => Ok(Education::update(&client, item).await?),
        DBObject::Kind(item) => Ok(Kind::update(&client, item).await?),
        DBObject::Post(item) => Ok(Post::update(&client, item).await?),
        DBObject::Practice(item) => Ok(Practice::update(&client, item).await?),
        DBObject::Rank(item) => Ok(Rank::update(&client, item).await?),
        DBObject::Scope(item) => Ok(Scope::update(&client, item).await?),
        DBObject::Siren(item) => Ok(Siren::update(&client, *item).await?),
        DBObject::SirenType(item) => Ok(SirenType::update(&client, item).await?),
        _ => Err(ServiceError::BadRequest("bad item object".to_string())),
    }
}

async fn post_item(
    name: &str,
    params: web::Json<DBObject>,
    client: &Client,
) -> Result<DBObject, ServiceError> {
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
        ("education", DBObject::Education(item)) => {
            Ok(DBObject::Education(Education::insert(client, item).await?))
        }
        ("kind", DBObject::Kind(item)) => Ok(DBObject::Kind(Kind::insert(client, item).await?)),
        ("post", DBObject::Post(item)) => Ok(DBObject::Post(Post::insert(client, item).await?)),
        ("practice", DBObject::Practice(item)) => {
            Ok(DBObject::Practice(Practice::insert(client, item).await?))
        }
        ("rank", DBObject::Rank(item)) => Ok(DBObject::Rank(Rank::insert(client, item).await?)),
        ("scope", DBObject::Scope(item)) => Ok(DBObject::Scope(Scope::insert(client, item).await?)),
        ("siren", DBObject::Siren(item)) => Ok(DBObject::Siren(Box::new(
            Siren::insert(client, *item).await?,
        ))),
        ("siren_type", DBObject::SirenType(item)) => {
            Ok(DBObject::SirenType(SirenType::insert(client, item).await?))
        }
        _ => Err(ServiceError::BadRequest(format!("bad path {}", name))),
    }
}

async fn delete_item(client: &Client, name: &str, id: i64) -> Result<u64, ServiceError> {
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
        _ => Err(ServiceError::BadRequest(format!("bad path {:?}", name))),
    }
}

fn http_result_list(res: Result<DBObject, ServiceError>) -> HttpResponse {
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

fn http_result_item(res: Result<DBObject, ServiceError>) -> HttpResponse {
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

pub async fn get_list_name(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    // let a = check_auth(id);
    // a?;
    let client = db.get().await?;
    let res = get_list(&path, &client).await;
    Ok(http_result_list(res))
}

pub async fn get_name_id(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, i64)>,
) -> Result<HttpResponse, ServiceError> {
    // let a = check_auth(id);
    // a?;
    let client = db.get().await?;
    let res = get_item(&Item::from(path.into_inner()), &client).await;
    Ok(http_result_item(res))
}

pub async fn post_name_id(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, i64)>,
    params: web::Json<DBObject>,
) -> Result<HttpResponse, ServiceError> {
    // let a = check_auth(id);
    // a?;
    let client = db.get().await?;
    let res = post_item(&path.0, params, &client).await;
    Ok(http_result_item(res))
}

pub async fn delete_name_id(
    // id: Identity,
    db: web::Data<Pool>,
    path: web::Path<(String, i64)>,
) -> Result<HttpResponse, ServiceError> {
    // let a = check_auth(id);
    //     a?;
    let client = db.get().await?;
    let res = delete_item(&client, &path.0, path.1).await;
    Ok(match res {
        Ok(_res) => HttpResponse::Ok().json(json!({
            "data": Null,
            "error": Null,
            "ok": true
        })),
        Err(err) => HttpResponse::Ok().json(json!({
            "data": Null,
            "error": err.to_string(),
            "ok": false
        })),
    })
}
