use std::collections::BinaryHeap;

use crate::{
    order::{Order, Side, Transaction},
    utils,
};

pub struct OrderBook {
    security: String,
    buys: BinaryHeap<Order>,
    sells: BinaryHeap<Order>,
    pub transactions: Vec<Transaction>,
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

    pub fn place(&mut self, order: Order) {
        if order.side == Side::Buy {
            self.place_buy(order);
        } else {
            self.place_sell(order);
        }
    }

    pub fn buy_orders(&self) -> usize {
        self.buys.len()
    }
    pub fn sell_orders(&self) -> usize {
        self.sells.len()
    }

    // TODO: optimize so we don't have to remove
    // and replace on heap each time
    fn place_buy(&mut self, mut order: Order) {
        assert!(order.side == Side::Buy);
        loop {
            if order.size == 0 {
                break;
            }
            let top = self.sells.pop();
            if top.is_none() {
                break;
            }
            let mut top_order = top.unwrap();
            if top_order.price > order.price {
                self.sells.push(top_order);
                break;
            }
            let size_matched = top_order.size.min(order.size);
            top_order.size -= size_matched;
            order.size -= size_matched;

            self.transactions.push(Transaction {
                from: top_order.creator.clone(),
                to: order.creator.clone(),
                security: self.security.clone(),
                size: size_matched,
                price: top_order.price,
                ts: utils::now(),
            });

            if top_order.size != 0 {
                self.sells.push(top_order);
            }
        }

        if order.size > 0 {
            self.buys.push(order);
        }
    }

    fn place_sell(&mut self, mut order: Order) {
        assert!(order.side == Side::Sell);
        loop {
            if order.size == 0 {
                break;
            }
            let top = self.buys.pop();
            if top.is_none() {
                break;
            }
            let mut top_order = top.unwrap();
            if top_order.price < order.price {
                self.buys.push(top_order);
                break;
            }
            let size_matched = top_order.size.min(order.size);
            top_order.size -= size_matched;
            order.size -= size_matched;

            self.transactions.push(Transaction {
                from: order.creator.clone(),
                to: top_order.creator.clone(),
                security: self.security.clone(),
                size: size_matched,
                price: top_order.price,
                ts: utils::now(),
            });

            if top_order.size != 0 {
                self.buys.push(top_order);
            }
        }

        if order.size > 0 {
            self.sells.push(order);
        }
    }
}
