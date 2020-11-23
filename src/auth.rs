use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
// use serde::{Deserialize, Serialize};

use crate::error::ServiceError;

// #[derive(Deserialize, Serialize)]
// pub struct Auth {
//     username: String,
//     password: String,
// }

pub async fn bearer_auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

fn validate_token(str: &str) -> Result<bool, ServiceError> {
    if str.eq("a-secure-token") {
        return Ok(true);
    }
    return Err(ServiceError::FailedAuth);
}

// fn validate_credentials(user_id: &str, user_password: &str) -> Result<bool, std::io::Error> {
//     if user_id.eq("karl") && user_password.eq("password") {
//         return Ok(true);
//     }
//     return Err(std::io::Error::new(
//         std::io::ErrorKind::Other,
//         "Authentication failed!",
//     ));
// }

// pub fn login(id: Identity, params: web::Json<Auth>) -> HttpResponse {
//     let secret_key = dotenv::var("SECRET_KEY").unwrap();
//     let auth: Auth = params.into_inner();
//     let mut s = auth.username;
//     s.push_str(&auth.password);
//     if base64::encode(s.as_bytes()) == secret_key {
//         id.remember("Member".to_owned());
//     } else {
//         id.forget();
//     }
//     check(id)
// }

// pub fn logout(id: Identity) -> HttpResponse {
//     id.forget();
//     HttpResponse::Ok().json(json!({
//         "error": Null,
//         "user": Null
//     }))
// }

// pub fn check(id: Identity) -> HttpResponse {
//     match check_auth(id) {
//         Ok(user) => HttpResponse::Ok().json(json!({
//             "user": user,
//             "error": Null,
//         })),
//         Err(err) => HttpResponse::Ok().json(json!({
//             "user": Null,
//             "error": err,
//         })),
//     }
// }

// pub fn check_auth(id: Identity) -> Result<String, ServiceError> {
//     match id.identity() {
//         Some(i) => Ok(i),
//         None => Err(ServiceError::NotAuth),
//     }
// }
