use std::fmt;
use std::clone::Clone;

use actix::{spawn, Actor, Addr, Context, Handler, ResponseActFuture, fut::wrap_future};
use actix_web::web;
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
// use log::info;
// use serde_json::json;

use rpel::certificate::{Certificate, CertificateList};
use rpel::company::{Company, CompanyList};
use rpel::contact::{Contact, ContactList};
use rpel::department::{Department, DepartmentList};
use rpel::education::{Education, EducationList, EducationShort};
use rpel::get_pool;
use rpel::kind::{Kind, KindList};
use rpel::post::{Post, PostList};
use rpel::practice::{Practice, PracticeList, PracticeShort};
use rpel::rank::{Rank, RankList};
use rpel::scope::{Scope, ScopeList};
use rpel::select::SelectItem;
use rpel::siren::{Siren, SirenList};
use rpel::siren_type::{SirenType, SirenTypeList};

use crate::error::ServiceError;
use crate::server::{Msg, Server};

#[derive(Serialize)]
pub struct WsMsg {
    pub name: String,
    pub object: DBObject,
    pub error: String,
}

#[derive(Clone)]
pub struct DB {
    pool: Pool,
    server: Addr<Server>,
}

impl Actor for DB {
    type Context = Context<Self>;
}

impl DB {
    fn new(server: Addr<Server>) -> DB {
        let pool = get_pool();
        let fut = async move {
            get_pool().get().await.expect("DB connection failed");
        };
        spawn(fut);
        DB { pool, server }
    }

    async fn client(&self) -> Result<Client, ServiceError> {
        Ok(self.pool.get().await?)
    }

    async fn get_reply(self, message: String) -> Result<DBObject, ServiceError> {
        let cmd: Command = serde_json::from_str(&message)?;
        let client = self.client().await?;
        let dbo = match cmd {
            Command::Get(object) => match object {
                Object::Item(item) => get_item(&item, &client).await,
                Object::List(obj) => get_list(&obj, &client).await,
            },
            Command::Insert(dbobject) => insert_item(dbobject, &client).await,
            Command::Update(dbobject) => {
                update_item(dbobject, &client).await.map(|_| DBObject::Null)
            }
            Command::Delete(item) => delete_item(&item, &client).await.map(|_| DBObject::Null),
        };
        dbo
    }
}

impl Handler<Msg> for DB {
    type Result = ResponseActFuture<Self, Result<DBObject, ServiceError>>;

    fn handle(&mut self, msg: Msg, _: &mut Context<Self>) -> Self::Result {
        let message = msg.0;
        let this = self.clone();
        Box::new(wrap_future(this.get_reply(message)))
    }
}

// impl Msg {
//     pub fn from_obj(name: String, obj: Result<DBObject, ServiceError>) -> Msg {
//         match obj {
//             Ok(object) => Msg {
//                 name,
//                 object: object,
//                 error: String::new(),
//             },
//             Err(err) => Msg {
//                 name,
//                 object: DBObject::Null,
//                 error: err.to_string(),
//             },
//         }
//     }
// }

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
    Delete(Item),
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

// pub async fn ws_text(pool: Pool, text: String) -> Msg {
//     Msg::from_obj(text.clone(), get_msg(pool, text).await)
// }

async fn get_msg(pool: Pool, text: String) -> Result<DBObject, ServiceError> {
    let cmd: Command = serde_json::from_str(&text)?;
    let client = &pool.get().await?;
    match cmd {
        Command::Get(object) => match object {
            Object::Item(item) => get_item(&item, client).await,
            Object::List(obj) => get_list(&obj, client).await,
        },
        Command::Insert(dbobject) => insert_item(dbobject, &client).await,
        Command::Update(dbobject) => update_item(dbobject, &client).await.map(|_| DBObject::Null),
        Command::Delete(item) => delete_item(&item, &client).await.map(|_| DBObject::Null),
    }
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

async fn delete_item(item: &Item, client: &Client) -> Result<u64, ServiceError> {
    match item.name.as_str() {
        "certificate" => Ok(Certificate::delete(client, item.id).await?),
        "company" => Ok(Company::delete(client, item.id).await?),
        "contact" => Ok(Contact::delete(client, item.id).await?),
        "department" => Ok(Department::delete(client, item.id).await?),
        "education" => Ok(Education::delete(client, item.id).await?),
        "kind" => Ok(Kind::delete(client, item.id).await?),
        "post" => Ok(Post::delete(client, item.id).await?),
        "practice" => Ok(Practice::delete(client, item.id).await?),
        "rank" => Ok(Rank::delete(client, item.id).await?),
        "scope" => Ok(Scope::delete(client, item.id).await?),
        "siren" => Ok(Siren::delete(client, item.id).await?),
        "siren_type" => Ok(SirenType::delete(client, item.id).await?),
        _ => Err(ServiceError::BadRequest(format!(
            "bad path {:?}",
            item.name
        ))),
    }
}
