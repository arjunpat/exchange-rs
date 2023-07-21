use std::collections::{BinaryHeap, HashMap};

use crate::{
    order::{Order, Side, Trade},
    utils,
};

pub struct OrderBook {
    bids: BinaryHeap<Order>,
    asks: BinaryHeap<Order>,
    pub trades: Vec<Trade>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bids: BinaryHeap::new(),
            asks: BinaryHeap::new(),
            trades: Vec::new(),
        }
    }

    pub fn current_price(&self) -> u64 {
        match self.trades.last() {
            Some(transaction) => transaction.price,
            None => 0,
        }
    }

    pub fn num_bids(&self) -> usize {
        self.bids.len()
    }
    pub fn num_asks(&self) -> usize {
        self.asks.len()
    }

    pub fn get_depth(&self) -> (HashMap<u64, u64>, HashMap<u64, u64>) {
        // this is super slow but good enough for now
        let mut bids = HashMap::new();
        self.bids.iter().for_each(|o| {
            *bids.entry(o.price).or_insert(0) += o.size;
        });

        let mut asks = HashMap::new();
        self.asks.iter().for_each(|o| {
            *asks.entry(o.price).or_insert(0) += o.size;
        });

        (bids, asks)
    }

    pub fn place(&mut self, mut order: Order) -> &[Trade] {
        let (book, other_book) = match order.side {
            Side::Buy => (&mut self.bids, &mut self.asks),
            Side::Sell => (&mut self.asks, &mut self.bids),
        };

        let num_transactions = self.trades.len();

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

            let t = Trade {
                from,
                to,
                size: size_matched,
                price: top_order.price,
                ts: utils::now(),
            };
            self.trades.push(t);

            if top_order.size == 0 {
                other_book.pop();
            }
        }

        if order.size > 0 {
            book.push(order);
        }

        &self.trades[num_transactions..]
    }
}
