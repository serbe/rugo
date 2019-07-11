use dotenv::dotenv;
// use postgres::Connection;

use std::env;

pub fn get_connurl() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

// pub fn get_connection() -> Connection {
//     let conn_url = get_connurl();
//     Connection::connect(conn_url.clone(), postgres::TlsMode::None)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", conn_url))
// }
