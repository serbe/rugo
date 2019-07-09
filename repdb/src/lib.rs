use postgres::Connection;

use dotenv::dotenv;
use std::env;

pub mod certificate;
pub mod company;
pub mod contact;
pub mod department;
pub mod education;
pub mod email;
pub mod kind;
pub mod phone;
pub mod post;
pub mod practice;
pub mod rank;
pub mod scope;
pub mod siren;
pub mod siren_type;
pub mod select;

pub fn get_connurl() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_connection() -> Connection {
    let conn_url = get_connurl();
    Connection::connect(conn_url.clone(), postgres::TlsMode::None)
        .unwrap_or_else(|_| panic!("Error connecting to {}", conn_url))
}
