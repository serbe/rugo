use std::net::SocketAddr;

use deadpool_postgres::Pool;
use futures::StreamExt;
use futures_util::sink::SinkExt;
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

use auth::{check_auth, login};
use anyhow::Result;
use rpel::get_pool;
use services::get_response;
use users::Users;

mod auth;
mod dbo;
// mod error;
mod rpel;
mod services;
mod users;

async fn accept_connection(peer: SocketAddr, stream: TcpStream, pool: Pool, users: Users) {
    if let Err(e) = handle_connection(peer, stream, pool, &users).await {
        // match e {
        //  ttError::ConnectionClosed | ttError::Protocol(_) | ttError::Utf8 => (),
        //  err => println!("Error processing connection: {}", err),
        // }
        error!("Error processing connection: {}", e);
    }
}

async fn send_message(ws: &mut WebSocketStream<TcpStream>, response: Result<String>) -> Result<()> {
    match response {
        Ok(item) => ws.send(Message::Text(item)).await?,
        Err(err) => {
            info!("error {:?}", err);
            // ws.close(None).await?;
        }
    }
    Ok(())
}

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    pool: Pool,
    users: &Users,
) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    info!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        let text = msg.to_text()?;

        info!("ws_stream.next {:?}", &text);

        if let Ok(checked_data) = serde_json::from_str(text) {
            let a = send_message(&mut ws_stream, check_auth(users, checked_data).await).await;
            info!("A {:?}", a);
            a?;
        } else if let Ok(client_message) = serde_json::from_str(text) {
            let cm = send_message(
                &mut ws_stream,
                get_response(users, client_message, pool.clone()).await,
            )
            .await;
            info!("CM {:?}", cm);
            cm?;
        } else if let Ok(login_data) = serde_json::from_str(text) {
            let ld = send_message(&mut ws_stream, login(users, login_data).await).await;
            info!("LD {:?}", ld);
            ld?;
        } else {
            info!("unknown {:?}", &text);
        }
    }

    Ok(())
}

async fn run() -> Result<()> {
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    std::env::set_var("RUST_LOG", "rugo=info");
    env_logger::init();

    let pool = get_pool();
    let users = Users::new(&pool).await?;

    let mut listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _addr)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream, pool.clone(), users.clone()));
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run())
}
