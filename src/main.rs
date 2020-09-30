#![type_length_limit="22068819"]

use std::net::SocketAddr;

use deadpool_postgres::Pool;
use futures::{StreamExt, stream::SplitSink};
use warp::{Filter, ws::{Message, WebSocket}};
use futures_util::sink::SinkExt;
use log::{error, info};
use serde_json;

use auth::{check_auth, login};
use error::Result;
use rpel::get_pool;
use services::get_response;
use users::Users;

mod auth;
mod dbo;
mod error;
mod rpel;
mod services;
mod users;

async fn accept_connection(socket: WebSocket, users: Users, pool: Pool) {
    if let Err(e) = handle_connection(socket, &users, pool).await {
        // match e {
        //  ttError::ConnectionClosed | ttError::Protocol(_) | ttError::Utf8 => (),
        //  err => println!("Error processing connection: {}", err),
        // }
        error!("Error processing connection: {}", e);
    }
}

async fn send_message(tx: &mut SplitSink<WebSocket, Message>, response: Result<String>) -> Result<()> {
    match response {
        Ok(item) => tx.send(Message::text(item)).await?,
        Err(err) => {
            info!("error {:?}", err);
            tx.close().await?;
        }
    }
    Ok(())
}

async fn handle_connection(
    socket: WebSocket,
    users: &Users,
    pool: Pool,
) -> Result<()> {
    let (mut tx, mut rx) = socket.split();
    info!("New WebSocket connection: {:?} {:?}", tx, rx);

    while let Some(msg) = rx.next().await {
        let msg = msg?;
        let text = msg.to_str()?;


        if let Ok(checked_data) = serde_json::from_str(text) {
            send_message(&mut tx, check_auth(users, checked_data).await).await?;
        }
        if let Ok(client_message) = serde_json::from_str(text) {
            send_message(
                &mut tx,
                get_response(users, client_message, pool.clone()).await,
            )
            .await?;
        }
        if let Ok(login_data) = serde_json::from_str(text) {
            send_message(&mut tx, login(users, login_data).await).await?;
        }
    }

    Ok(())
}

async fn run_warp() -> Result<()> {
    std::env::set_var("RUST_LOG", "rugo=info");
    env_logger::init();

    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    let pool = get_pool();
    let users = Users::new(&pool).await?;

    let users = warp::any().map(move || users.clone());
    let pool = warp::any().map(move || pool.clone());

    let db_ws = warp::path("ws")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .and(users)
        .and(pool)
        .map(|ws: warp::ws::Ws, users, pool| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| accept_connection(socket, users, pool))
        });

    

    warp::serve(db_ws).run(addr.parse::<SocketAddr>()?).await;

    Ok(())

}

fn main() -> Result<()> {
    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_warp())
}
