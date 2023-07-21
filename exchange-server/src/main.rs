#![allow(dead_code)]

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::exchange::Exchange;

mod exchange;
mod order;
mod orderbook;
mod tests;
mod utils;

#[tokio::main]
async fn main() {
    let exch = Arc::new(Exchange::new());
    let exch_c = exch.clone();
    tokio::spawn(async move {
        exch_c.start().await;
    });
    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler))
        .with_state(exch);

    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(ip, 8080);
    println!("Listing on {}", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    Html(
        tokio::fs::read_to_string("./src/html/index.html")
            .await
            .unwrap(),
    )
}
async fn ws_handler(ws: WebSocketUpgrade, State(ex): State<Arc<Exchange>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, ex))
}

async fn handle_socket(mut socket: WebSocket, ex: Arc<Exchange>) {
    let _ = socket.send(Message::Ping(vec![])).await;

    let result = ex.handle_connection(&mut socket).await;
    if result.is_err() {
        println!("Failed connection: {:?}", result.unwrap_err());
    }
}
