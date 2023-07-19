#![allow(dead_code)]

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
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
    let exchange = Arc::new(Exchange::new());
    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler))
        .with_state(exchange);

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

async fn handle_socket(socket: WebSocket, ex: Arc<Exchange>) {
    if ex.handle_connection(socket).await.is_err() {
        println!("Failed connection");
    }
}
