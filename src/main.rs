use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{response::IntoResponse, routing::get, Router};
use axum_tungstenite::{Message, WebSocket, WebSocketUpgrade};
use futures_util::{SinkExt, StreamExt as _};

async fn handle_socket(socket: WebSocket) {
    println!("Handling socket");

    let (mut sender, mut receiver) = socket.split();

    while let Some(r) = receiver.next().await {
        match r {
            Ok(Message::Text(text)) => {
                if text.len() > 8 {
                    println!("Breaking the loop manually");
                    break;
                }
                println!("Replying");
                sender.send(Message::Text(text)).await.unwrap();
            }
            Ok(_) => {
                println!("Unsupported message type");
            }
            Err(e) => println!("{e}"),
        }
    }

    println!("Exiting");
}

pub async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

fn ws_router() -> Router {
    Router::new().route("/ws", get(handler))
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let server = tokio::spawn(
        axum::Server::bind(&SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            4242,
        ))
        .serve(ws_router().into_make_service()),
    );

    println!("Server started");

    server.await.unwrap().unwrap();
}
