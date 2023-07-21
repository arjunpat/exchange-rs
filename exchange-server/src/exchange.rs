use crate::{
    order::{Order, Side},
    orderbook::OrderBook,
    utils,
};
use anyhow::{Context, Result};
use axum::extract::ws::{Message, WebSocket};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::{
    sync::{broadcast, mpsc, Mutex},
    time::Duration,
};

pub struct Exchange {
    book: Mutex<OrderBook>,
    // update connected clients on all events
    broadcast: broadcast::Sender<ServerMsg>,
    // send order to exchange task
    order_sender: mpsc::Sender<Order>,
    // only for exchange_loop
    order_receiver: Mutex<mpsc::Receiver<Order>>,
    // id counter
    id_counter: Mutex<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ServerMsg {
    // set username at connection
    SetUsername {
        username: String,
    },
    Trade {
        price_cents: u64,
        size: u64,
        ts: u64,
    },
    Depths {
        bids: HashMap<u64, u64>,
        asks: HashMap<u64, u64>,
    },
}

impl From<ServerMsg> for Message {
    fn from(msg: ServerMsg) -> Self {
        // this should never fail
        let json = serde_json::to_string(&msg).expect("Failed to serialize json");
        Message::Text(json)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ClientMsg {
    // client Order
    Order {
        size: u64,
        price_cents: u64,
        buy: bool,
    },
}

impl Exchange {
    pub fn new() -> Self {
        let (order_sender, order_receiver) = mpsc::channel(32);
        let (broadcast, _) = broadcast::channel(32);

        Exchange {
            book: Mutex::new(OrderBook::new()),
            order_sender,
            order_receiver: Mutex::new(order_receiver),
            broadcast,
            id_counter: Mutex::new(1),
        }
    }

    pub async fn start(&self) {
        tokio::select! {
            _ = self.exchange_loop() => {}
            _ = self.send_depths_loop() => {}
        }
        println!("Exited start");
    }

    pub async fn send_depths_loop(&self) {
        loop {
            tokio::time::sleep(Duration::from_millis(250)).await;
            let (bids, asks);
            {
                (bids, asks) = self.book.lock().await.get_depth();
            }
            let _ = self.broadcast.send(ServerMsg::Depths { bids, asks });
        }
    }

    pub async fn exchange_loop(&self) {
        // also prevents two loops from ever existing
        let mut order_receiver = self.order_receiver.lock().await;
        let mut orders_received = 0;

        while let Some(order) = order_receiver.recv().await {
            orders_received += 1;
            if orders_received % 1000 == 0 {
                println!("Have received {} orders", orders_received);
            }
            {
                for t in self.book.lock().await.place(order) {
                    // can fail when there are no broadcast listeners
                    let _ = self.broadcast.send(ServerMsg::Trade {
                        price_cents: t.price,
                        size: t.size,
                        ts: t.ts,
                    });
                }
            } // essential to release the lock
        }
    }

    pub async fn handle_connection(&self, socket: &mut WebSocket) -> Result<()> {
        let username;
        {
            let mut id_counter = self.id_counter.lock().await;
            username = format!("user-{}", *id_counter);
            *id_counter += 1;
        } // release mutex lock

        // send client their username
        let msg = ServerMsg::SetUsername {
            username: username.clone(),
        };
        socket.send(msg.into()).await?;

        let mut broadcast_rx = self.broadcast.subscribe();

        loop {
            tokio::select! {
                update = broadcast_rx.recv() => {
                    socket.send(update?.into()).await?
                }
                msg = socket.recv() => {
                    if let Some(msg) = msg {
                        if let Ok(msg) = msg {
                            if let Message::Text(text) = msg { self.handle_client_msg(&username, text).await? }
                        } else {
                            println!("Client disconnected!!!!");
                            break;
                        }
                    } else {
                        println!("Stream has closed");
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn handle_client_msg(&self, username: &str, msg: String) -> Result<()> {
        let msg: ClientMsg = serde_json::from_str(&msg)
            .context(format!("Failed to deserialize client msg: {}", msg))?;
        // println!("received client messgae = {:?}", msg);

        match msg {
            ClientMsg::Order {
                size,
                price_cents,
                buy,
            } => {
                if size == 0 {
                    return Ok(());
                }
                let order = Order {
                    side: if buy { Side::Buy } else { Side::Sell },
                    created_at: utils::now(),
                    creator: username.to_owned(),
                    size,
                    price: price_cents,
                };

                // send to exchange tokio task/thread
                self.order_sender.send(order).await?;
            }
        }

        Ok(())
    }
}
