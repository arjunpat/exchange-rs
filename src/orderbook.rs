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

    pub fn buy_orders(&self) -> usize {
        self.buys.len()
    }
    pub fn sell_orders(&self) -> usize {
        self.sells.len()
    }

    pub fn place(&mut self, mut order: Order) {
        let (book, other_book) = match order.side {
            Side::Buy => (&mut self.buys, &mut self.sells),
            Side::Sell => (&mut self.sells, &mut self.buys),
        };

        loop {
            if order.size == 0 {
                break;
            }
            let top_order = other_book.peek();
            if top_order.is_none() {
                break;
            }
            let liquidity_unavailable = match order.side {
                Side::Buy => top_order.unwrap().price > order.price,
                Side::Sell => top_order.unwrap().price < order.price,
            };
            if liquidity_unavailable {
                break;
            }
            let size_matched = top_order.unwrap().size.min(order.size);

            other_book.peek_mut().unwrap().size -= size_matched;
            order.size -= size_matched;

            let top_order = other_book.peek().unwrap();

            let (from, to) = match order.side {
                Side::Sell => (order.creator.clone(), top_order.creator.clone()),
                Side::Buy => (top_order.creator.clone(), order.creator.clone()),
            };

            self.transactions.push(Transaction {
                from,
                to,
                security: self.security.clone(),
                size: size_matched,
                price: top_order.price,
                ts: utils::now(),
            });

            if top_order.size == 0 {
                other_book.pop();
            }
        }

        if order.size > 0 {
            book.push(order);
        }
    }
}
