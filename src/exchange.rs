use crate::{
    order::{Order, Side, Transaction},
    orderbook::OrderBook,
    utils,
};
use anyhow::{Context, Result};
use axum::extract::ws::{Message, WebSocket};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};

pub struct Exchange {
    // update connected clients on all events
    broadcast: broadcast::Sender<ServerMsg>,
    // send order to exchange task
    order_sender: mpsc::Sender<(String, Order)>,
    // list of traded securities
    securities_traded: Arc<RwLock<HashSet<String>>>,
    // id counter
    id_counter: Arc<Mutex<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ServerMsg {
    // set username at connection
    SetUsername { username: String },
    // transaction
    Transaction(Transaction),
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
        security: String,
        size: i64,
        price: f64,
        buy: bool,
    },
}

impl Exchange {
    pub fn new() -> Self {
        let (order_sender, mut order_receiver) = mpsc::channel(32);
        let (broadcast, _) = broadcast::channel(32);

        let exch = Exchange {
            order_sender,
            broadcast,
            securities_traded: Arc::new(RwLock::new(HashSet::new())),
            id_counter: Arc::new(Mutex::new(1)),
        };

        let s_traded_c = exch.securities_traded.clone();
        let broadcast_c = exch.broadcast.clone();
        tokio::spawn(async move {
            let mut orderbooks: HashMap<&str, OrderBook> = HashMap::new();

            orderbooks.insert("AAPL", OrderBook::new("AAPL"));

            {
                let mut set = s_traded_c.write().await;
                for each in orderbooks.keys() {
                    set.insert((*each).to_string());
                }
            } // release write lock

            while let Some((security, order)) = order_receiver.recv().await {
                // println!("GOT an order = {:?}", (&security, &order));
                // there should always be a book bc prechecked before sent
                let book = orderbooks.get_mut(security.as_str()).unwrap();
                for transaction in book.place(order) {
                    broadcast_c
                        .send(ServerMsg::Transaction(transaction.clone()))
                        .unwrap();
                }
            }
        });

        exch
    }

    pub async fn handle_connection(&self, mut socket: WebSocket) -> Result<()> {
        let mut id_counter = self.id_counter.lock().await;
        let username = format!("user-{}", *id_counter);
        *id_counter += 1;

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
                            self.handle_client_msg(&username, msg).await?;
                        } else {
                            // client disconnected
                            println!("Client disconnected!!!!");
                            break;
                        }
                    } else {
                        println!("Client disconnected here!!");
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn handle_client_msg(&self, username: &str, msg: Message) -> Result<()> {
        let msg: ClientMsg =
            serde_json::from_str(&msg.into_text()?).context("Failed to deserialize client msg")?;
        // println!("received client messgae = {:?}", msg);

        match msg {
            ClientMsg::Order {
                security,
                size,
                price,
                buy,
            } => {
                let order = Order {
                    side: if buy { Side::Buy } else { Side::Sell },
                    created_at: utils::now(),
                    creator: username.to_owned(),
                    size,
                    price,
                };

                // check security is traded
                if self.securities_traded.read().await.contains(&security) {
                    // send to exchange tokio task/thread
                    self.order_sender.send((security, order)).await?;
                } else {
                    println!("Security {security} is not traded");
                }
            }
        }

        Ok(())
    }
}
