use std::collections::BinaryHeap;

use crate::order::{Order, Transaction};

pub struct OrderBook {
    security: String,
    buys: BinaryHeap<Order>,
    sells: BinaryHeap<Order>,
    transactions: Vec<Transaction>,
}

impl OrderBook {
    pub fn new(security: &str) -> OrderBook {
        return OrderBook {
            security: security.to_owned(),
            buys: BinaryHeap::new(),
            sells: BinaryHeap::new(),
            transactions: Vec::new(),
        };
    }

    pub fn current_price(&self) -> f64 {
        match self.transactions.last() {
            Some(transaction) => transaction.price,
            None => 0.0,
        }
    }
}
