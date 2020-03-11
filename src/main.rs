// use std::io;
// use std::time::{Duration, Instant};

use anyhow::Error;

// use auth::{check, login, logout};
// use db::{
//     delete_name_id,
//     get_name_children,
//     get_name_command,
//     get_name_id,
//     //post_name_id,
// };
// use db::test_post_name_id;

// use rpel::get_pool;

use std::{
    // collections::HashMap,
    // env,
    // io::Error as IoError,
    net::SocketAddr,
    // sync::{Arc, Mutex},
};

use futures::{
    // channel::mpsc::{unbounded, UnboundedSender},
    // future,
    // pin_mut,
    // stream::TryStreamExt,
    StreamExt,
};
use futures_util::{SinkExt};
// use log::{error, info};
use tokio_tungstenite::{accept_async, tungstenite::Error as ttError};
use tokio::net::{TcpListener, TcpStream};
// use tungstenite::{protocol::Message};

// type Tx = UnboundedSender<Message>;
// type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

// mod auth;
// mod db;

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

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            ttError::ConnectionClosed | ttError::Protocol(_) | ttError::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<(), tungstenite::Error> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let _secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    // let pool = get_pool();

    // std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    // env_logger::init();

    let addr = "127.0.0.1:8080".to_string();

    let mut listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    while let Ok((stream, _addr)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
            println!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }

    // let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.

    // Let's spawn the handling of each connection in a separate task.
    // while let Ok((stream, addr)) = listener.accept().await {
    //     tokio::spawn(handle_connection(state.clone(), stream, addr));
    // }

    Ok(())
}

//64475  94