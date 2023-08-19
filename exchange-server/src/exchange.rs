use crate::orderbook::Book;
use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, sync::atomic::AtomicU32};
use tokio::sync::broadcast;

pub struct Exchange {
    book: RefCell<Book>,
    // update connected clients on all events
    broadcast: broadcast::Sender<ServerMsg>,
    // id counter
    id_counter: AtomicU32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    price_cents: u32,
    quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ServerMsg {
    SetUsername(String),
    Trades(Vec<Trade>),
    DepthsBook {
        bids: HashMap<u32, u32>,
        asks: HashMap<u32, u32>,
    },
}

impl From<ServerMsg> for Message {
    fn from(msg: ServerMsg) -> Self {
        let json = serde_json::to_string(&msg).expect("Failed to serialize json");
        Message::Text(json)
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum ClientMsg {
    Order {
        size: u32,
        price_cents: u32,
        buy: bool,
    },
}

impl Exchange {
    pub fn new() -> Self {
        let (broadcast, _) = broadcast::channel(32);

        Self {
            book: RefCell::new(Book::new()),
            broadcast,
            id_counter: AtomicU32::new(1),
        }
    }
}
