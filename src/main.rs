use std::net::SocketAddr;

use anyhow::{Error, Result};
use deadpool_postgres::Pool;
use futures::StreamExt;
use futures_util::SinkExt;
use log::{error, info};
use serde_json;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message;

use rpel::get_pool;

use db::Command;

// type Tx = UnboundedSender<Message>;
// type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

// mod auth;
mod db;

// async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
//     println!("Incoming TCP connection from: {}", addr);

//     let ws_stream = tokio_tungstenite::accept_async(raw_stream)
//         .await
//         .expect("Error during the websocket handshake occurred");
//     println!("WebSocket connection established: {}", addr);

//     // Insert the write part of this peer to the peer map.
//     let (tx, rx) = unbounded();
//     peer_map.lock().unwrap().insert(addr, tx);

//     let (outgoing, incoming) = ws_stream.split();

//     let broadcast_incoming = incoming.try_for_each(|msg| {
//         println!(
//             "Received a message from {}: {}",
//             addr,
//             msg.to_text().unwrap()
//         );
//         let peers = peer_map.lock().unwrap();

//         // We want to broadcast the message to everyone except ourselves.
//         let broadcast_recipients = peers
//             .iter()
//             .filter(|(peer_addr, _)| peer_addr != &&addr)
//             .map(|(_, ws_sink)| ws_sink);

//         for recp in broadcast_recipients {
//             recp.unbounded_send(msg.clone()).unwrap();
//         }

//         future::ok(())
//     });

//     let receive_from_others = rx.map(Ok).forward(outgoing);

//     pin_mut!(broadcast_incoming, receive_from_others);
//     future::select(broadcast_incoming, receive_from_others).await;

//     println!("{} disconnected", &addr);
//     peer_map.lock().unwrap().remove(&addr);
// }

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

    println!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let cmd: Command = serde_json::from_str(msg?.to_text()?)?;
        match cmd {
            Command::Get(object) => {
                let msg = db::get_object(&object, pool.clone()).await;
                let js = serde_json::to_string(&msg)?;
                ws_stream.send(Message::Text(js)).await?;
            }
            Command::Set(_db_object) => (),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let pool = get_pool();

    std::env::set_var("RUST_LOG", "rugo=info");
    env_logger::init();

    let addr = "127.0.0.1:9090".to_string();

    let mut listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _addr)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream, pool.clone()));
    }

    // let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.

    // Let's spawn the handling of each connection in a separate task.
    // while let Ok((stream, addr)) = listener.accept().await {
    //     tokio::spawn(handle_connection(state.clone(), stream, addr));
    // }

    Ok(())
}
