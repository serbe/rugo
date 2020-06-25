use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value::Null};
use crate::error::ServiceError;

#[derive(Deserialize, Serialize)]
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
    match check_auth(id) {
        Ok(user) => HttpResponse::Ok().json(json!({
            "user": user,
            "error": Null,
        })),
        Err(err) => HttpResponse::Ok().json(json!({
            "user": Null,
            "error": err,
        })),
    }
}

pub fn check_auth(id: Identity) -> Result<String, ServiceError> {
    match id.identity() {
        Some(i) => Ok(i),
        None => Err(ServiceError::NotAuth),
    }
}
