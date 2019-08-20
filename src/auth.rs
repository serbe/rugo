use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value::Null};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    username: String,
    password: String,
}

pub fn login(id: Identity, params: web::Json<Auth>) -> HttpResponse {
    let secret_key = dotenv::var("SECRET_KEY").unwrap();
    let auth: Auth = params.into_inner();
    let mut s = auth.username;
    s.push_str(&auth.password);
    if base64::encode(s.as_bytes()) == secret_key {
        id.remember("Member".to_owned());
    } else {
        id.forget();
    }
    check(id)
}

pub fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().json(json!({
        "error": Null,
        "user": Null
    }))
}

pub fn check(id: Identity) -> HttpResponse {
    if let Ok(user) = check_auth(id) {
        HttpResponse::Ok().json(json!({
            "error": Null,
            "user": user
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "error": "Not auth".to_owned(),
            "user": Null
        }))
    }
}

pub fn check_auth(id: Identity) -> Result<String, String> {
    if let Some(i) = id.identity() {
        // println!("auth {}", i);
        Ok(i)
    } else {
        // println!("not auth");
        Err("Not auth".to_owned())
    }
}
