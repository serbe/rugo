use std::clone::Clone;
use std::collections::HashMap;
use std::fmt;
use std::iter;
// use std::sync::Mutex;

use actix::{fut, Actor, Addr, Context, Handler, ResponseActFuture};
use deadpool_postgres::{Client, Pool};
use once_cell::sync::OnceCell;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};

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
use rpel::user::UserList;

use crate::auth::check;
use crate::error::ServiceError;
use crate::server::{Msg, Server};

#[derive(Clone)]
pub struct UserData {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub role: i64,
}

impl UserData {
    pub fn permissions(&self, command: Command) -> Result<Command, ServiceError> {
        if match &command {
            Command::Get(_) => self.role >> 1 > 0,
            Command::Insert(_) => self.role >> 2 > 0,
            Command::Update(_) => self.role >> 3 > 0,
            Command::Delete(_) => self.role >> 4 > 0,
        } {
            Ok(command)
        } else {
            Err(ServiceError::NotPermission)
        }
    }
}

static USERS: OnceCell<HashMap<String, UserData>> = OnceCell::new();

pub async fn global_init() -> Result<(), ServiceError> {
    let mut rng = thread_rng();
    let client = get_pool().get().await?;
    let users = UserList::get_all(&client)
        .await
        .expect("get UserList failed");
    let mut hash_map = HashMap::new();
    for user in users {
        let key = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(20)
            .collect();
        hash_map.insert(
            key,
            UserData {
                id: user.id,
                name: user.name.clone(),
                key: user.key.clone(),
                role: user.role,
            },
        );
    }
    // let mutex = Mutex::new(hash_map);
    let _g_users = USERS.set(hash_map).expect_err("error");
    Ok(())
}

pub fn get_user(key: &str) -> Option<UserData> {
    let users = USERS.get()?;
    // let users = mutex.lock().ok()?;
    let user = users.get(key)?;
    Some(user.clone())
}

pub fn get_key(username: &str, userkey: &str) -> Option<String> {
    println!("pre users");
    let users = USERS.get()?;
    // println!("mutex");
    // let users = mutex.lock().ok()?;
    println!("users");
    let key = users
        .iter()
        .find(|(_key, user)| user.name == username && user.key == userkey)
        .map(|(key, _user)| key)?;
    println!("key");
    Some(key.clone())
}

#[derive(Serialize)]
pub struct WsMsg {
    pub name: String,
    pub object: DBObject,
    pub error: String,
}

impl WsMsg {
    pub fn from_dbo(name: String, dbo: Result<DBObject, ServiceError>) -> WsMsg {
        match dbo {
            Ok(object) => WsMsg {
                name,
                object: object,
                error: String::new(),
            },
            Err(err) => WsMsg {
                name,
                object: DBObject::Null,
                error: err.to_string(),
            },
        }
    }
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
    pub fn new(server: Addr<Server>) -> DB {
        let pool = get_pool();
        DB { pool, server }
    }

    async fn client(&self) -> Result<Client, ServiceError> {
        Ok(self.pool.get().await?)
    }

    async fn get_reply(self, message: String) -> Result<String, ServiceError> {
        let client_message: ClientMessage = serde_json::from_str(&message)?;
        let cmd: Command = check(client_message)?;
        let client = self.client().await?;
        let msg = match cmd {
            Command::Get(object) => match object {
                Object::Item(item) => {
                    WsMsg::from_dbo(item.name.clone(), get_item(&item, &client).await)
                }
                Object::List(obj) => WsMsg::from_dbo(obj.clone(), get_list(&obj, &client).await),
            },
            Command::Insert(dbobject) => WsMsg::from_dbo(
                String::new(),
                Ok(DBObject::Res(insert_item(dbobject, &client).await?)),
            ),
            Command::Update(dbobject) => WsMsg::from_dbo(
                String::new(),
                Ok(DBObject::Res(update_item(dbobject, &client).await?)),
            ),
            Command::Delete(item) => WsMsg::from_dbo(
                String::new(),
                Ok(DBObject::Res(delete_item(&item, &client).await?)),
            ),
        };
        Ok(serde_json::to_string(&msg)?)
    }
}

impl Handler<Msg> for DB {
    type Result = ResponseActFuture<Self, Result<String, ServiceError>>;

    fn handle(&mut self, msg: Msg, _: &mut Context<Self>) -> Self::Result {
        // println!("DB MESSAGE: {:?}", msg.0);
        let message = msg.0;
        let this = self.clone();
        Box::new(fut::wrap_future(this.get_reply(message)))
    }
}

#[derive(Deserialize)]
pub struct ClientMessage {
    pub command: Command,
    pub addon: String,
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
    Insert(DBObject),
    Update(DBObject),
    Delete(Item),
}

#[derive(Deserialize, Serialize)]
pub enum DBObject {
    Null,
    Res(i64),
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

// impl DBObject {
//     fn name(&self) -> String {
//         match self {
//             DBObject::Res(_) => String::new(),
//             DBObject::Null => String::new(),
//             DBObject::Certificate(_) => String::from("Certificate"),
//             DBObject::CertificateList(_) => String::from("CertificateList"),
//             DBObject::Company(_) => String::from("Company"),
//             DBObject::CompanyList(_) => String::from("CompanyList"),
//             DBObject::Contact(_) => String::from("Contact"),
//             DBObject::ContactList(_) => String::from("ContactList"),
//             DBObject::Department(_) => String::from("Department"),
//             DBObject::DepartmentList(_) => String::from("DepartmentList"),
//             DBObject::Education(_) => String::from("Education"),
//             DBObject::EducationList(_) => String::from("EducationList"),
//             DBObject::EducationShort(_) => String::from("EducationShort"),
//             DBObject::Kind(_) => String::from("Kind"),
//             DBObject::KindList(_) => String::from("KindList"),
//             DBObject::Post(_) => String::from("Post"),
//             DBObject::PostList(_) => String::from("PostList"),
//             DBObject::Practice(_) => String::from("Practice"),
//             DBObject::PracticeList(_) => String::from("PracticeList"),
//             DBObject::PracticeShort(_) => String::from("PracticeShort"),
//             DBObject::Rank(_) => String::from("Rank"),
//             DBObject::RankList(_) => String::from("RankList"),
//             DBObject::Scope(_) => String::from("Scope"),
//             DBObject::ScopeList(_) => String::from("ScopeList"),
//             DBObject::SelectItem(_) => String::from("SelectItem"),
//             DBObject::Siren(_) => String::from("Siren"),
//             DBObject::SirenList(_) => String::from("SirenList"),
//             DBObject::SirenType(_) => String::from("SirenType"),
//             DBObject::SirenTypeList(_) => String::from("SirenTypeList"),
//         }
//     }
// }

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Item(i) => write!(f, "Item {} {}", i.id, i.name),
            Object::List(s) => write!(f, "List {}", s),
        }
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

async fn insert_item(object: DBObject, client: &Client) -> Result<i64, ServiceError> {
    match object {
        DBObject::Certificate(item) => Ok(Certificate::insert(&client, item).await?.id),
        DBObject::Company(item) => Ok(Company::insert(&client, *item).await?.id),
        DBObject::Contact(item) => Ok(Contact::insert(&client, *item).await?.id),
        DBObject::Department(item) => Ok(Department::insert(&client, item).await?.id),
        DBObject::Education(item) => Ok(Education::insert(&client, item).await?.id),
        DBObject::Kind(item) => Ok(Kind::insert(&client, item).await?.id),
        DBObject::Post(item) => Ok(Post::insert(&client, item).await?.id),
        DBObject::Practice(item) => Ok(Practice::insert(&client, item).await?.id),
        DBObject::Rank(item) => Ok(Rank::insert(&client, item).await?.id),
        DBObject::Scope(item) => Ok(Scope::insert(&client, item).await?.id),
        DBObject::Siren(item) => Ok(Siren::insert(&client, *item).await?.id),
        DBObject::SirenType(item) => Ok(SirenType::insert(&client, item).await?.id),
        _ => Err(ServiceError::BadRequest("bad item object".to_string())),
    }
}

async fn update_item(object: DBObject, client: &Client) -> Result<i64, ServiceError> {
    let res = match object {
        DBObject::Certificate(item) => Certificate::update(&client, item).await,
        DBObject::Company(item) => Company::update(&client, *item).await,
        DBObject::Contact(item) => Contact::update(&client, *item).await,
        DBObject::Department(item) => Department::update(&client, item).await,
        DBObject::Education(item) => Education::update(&client, item).await,
        DBObject::Kind(item) => Kind::update(&client, item).await,
        DBObject::Post(item) => Post::update(&client, item).await,
        DBObject::Practice(item) => Practice::update(&client, item).await,
        DBObject::Rank(item) => Rank::update(&client, item).await,
        DBObject::Scope(item) => Scope::update(&client, item).await,
        DBObject::Siren(item) => Siren::update(&client, *item).await,
        DBObject::SirenType(item) => SirenType::update(&client, item).await,
        _ => Err(ServiceError::BadRequest("bad item object".to_string()))?,
    }?;
    Ok(res as i64)
}

async fn delete_item(item: &Item, client: &Client) -> Result<i64, ServiceError> {
    let res = match item.name.as_str() {
        "certificate" => Certificate::delete(client, item.id).await,
        "company" => Company::delete(client, item.id).await,
        "contact" => Contact::delete(client, item.id).await,
        "department" => Department::delete(client, item.id).await,
        "education" => Education::delete(client, item.id).await,
        "kind" => Kind::delete(client, item.id).await,
        "post" => Post::delete(client, item.id).await,
        "practice" => Practice::delete(client, item.id).await,
        "rank" => Rank::delete(client, item.id).await,
        "scope" => Scope::delete(client, item.id).await,
        "siren" => Siren::delete(client, item.id).await,
        "siren_type" => SirenType::delete(client, item.id).await,
        _ => Err(ServiceError::BadRequest(format!(
            "bad path {:?}",
            item.name
        )))?,
    }?;
    Ok(res as i64)
}
