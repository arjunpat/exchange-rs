#![warn(rust_2018_idioms)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

mod exchange;
mod orderbook;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler));

    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(ip, 8080);
    println!("Listening on {}", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("tbd")
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    let _ = socket.send(Message::Ping(vec![])).await;
}
