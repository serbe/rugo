use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    username: String,
    password: String,
}

pub fn login(id: Identity, params: web::Json<Auth>,) -> HttpResponse {
    let secret_key = dotenv::var("SECRET_KEY").unwrap();
    let auth: Auth = params.into_inner();

    let mut s = auth.username;
    s.push_str(&auth.password);
    if base64::encode(s.as_bytes()) == secret_key {
        id.remember("Member".to_owned());
        HttpResponse::Found().header("location", "/").finish()
    } else {
        id.forget();
        HttpResponse::Found().header("location", "/").finish()
    }
}

pub fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}

pub fn check_auth(id: Identity) -> Result<(), String> {
    if let Some(i) = id.identity() {
        println!("auth {}", i);
        Ok(())
    } else {
        println!("not auth");
        Err("Not auth".to_owned())
    }
}