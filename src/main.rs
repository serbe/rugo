// use std::io;
use dotenv::dotenv;

// use auth::{check, login, logout};
// use db::{
//     delete_name_id, get_manager, get_name_children, get_name_command, get_name_id, post_name_id,
// };
// use websockets::ws_index;

use db::get_pool;
use server::Server;

// mod auth;
// mod error;
mod certificate;
mod company;
mod contact;
mod db;
mod department;
mod education;
mod email;
mod kind;
mod phone;
mod post;
mod practice;
mod rank;
mod scope;
mod select;
mod server;
mod siren;
mod siren_type;
mod tcc;

fn main() {
    dotenv().ok();
    // let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let ws_addr = dotenv::var("WS_ADDR").expect("WS_ADDR must be set");
    let pool = get_pool();

    // std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    ws::Builder::new()
        .build(|out: ws::Sender| Server {
            out: out,
            pool: pool.clone(),
        })
        .expect("failed build ws builder")
        .listen(ws_addr)
        .expect("failed listen ws");
}
