#![type_length_limit="1297109"]

use std::net::SocketAddr;

use anyhow::{Error, Result};
use deadpool_postgres::Pool;
use futures::StreamExt;
use futures_util::sink::SinkExt;
use log::{error, info};
use serde_json;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use auth::{check_auth, login};
use rpel::get_pool;
use services::get_response;
use users::global_init;

mod auth;
mod dbo;
mod error;
mod rpel;
mod services;
mod users;

async fn accept_connection(peer: SocketAddr, stream: TcpStream, pool: Pool) {
    if let Err(e) = handle_connection(peer, stream, pool).await {
        // match e {
        //  ttError::ConnectionClosed | ttError::Protocol(_) | ttError::Utf8 => (),
        //  err => println!("Error processing connection: {}", err),
        // }
        error!("Error processing connection: {}", e);
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream, pool: Pool) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    info!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if let Ok(client_message) = serde_json::from_str(msg.to_text()?) {
            let response = get_response(client_message, pool.clone()).await?;
            ws_stream.send(Message::Text(response)).await?;
        } else if let Ok(checked_data) = serde_json::from_str(msg.to_text()?) {
            let response = check_auth(checked_data).await?;
            ws_stream.send(Message::Text(response)).await?;
        } else if let Ok(login_data) = serde_json::from_str(msg.to_text()?) {
            let response = login(login_data).await?;
            ws_stream.send(Message::Text(response)).await?;
        } else {
            info!("wrong ws text: {}", msg.to_text()?)
        }
    }

    Ok(())
}

async fn run() -> Result<(), Error> {
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    std::env::set_var("RUST_LOG", "rugo=info");
    env_logger::init();
    global_init().await?;

    let pool = get_pool();

    let mut listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _addr)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream, pool.clone()));
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run())
}
