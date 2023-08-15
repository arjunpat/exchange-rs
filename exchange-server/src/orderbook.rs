use std::collections::{BTreeMap, BinaryHeap};

use crate::{
    order::{Order, Side, Trade},
    utils,
};

pub struct OrderBook {
    bids: BinaryHeap<Order>,
    asks: BinaryHeap<Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BinaryHeap::new(),
            asks: BinaryHeap::new(),
        }
    }

    pub fn place_market(&mut self, mut order: Order) {
        let (book, other_book) = match order.side() {
            Side::Buy => (&mut self.bids, &mut self.asks),
            Side::Sell => (&mut self.asks, &mut self.bids),
        };
    }

    pub fn place_limit(&mut self, mut order: Order) {
        let (book, other_book) = match order.side() {
            Side::Buy => (&mut self.bids, &mut self.asks),
            Side::Sell => (&mut self.asks, &mut self.bids),
        };

        // all_or_none is not yet supported for limit orders
        assert!(!order.all_or_none());

        let ts = utils::now();

        loop {
            if order.quantity() == 0 {
                break;
            }
            let top_order = other_book.peek();
            if top_order.is_none() {
                break;
            }
            let liquidity_unavailable = match order.side() {
                Side::Buy => top_order.unwrap().price() > order.price(),
                Side::Sell => top_order.unwrap().price() < order.price(),
            };
            if liquidity_unavailable {
                break;
            }
            let size_matched = top_order.unwrap().quantity().min(order.quantity());

            other_book.peek_mut().unwrap().reduce_quantity(size_matched);
            order.reduce_quantity(size_matched);

            let top_order = other_book.peek().unwrap();

            let (from, to) = match order.side() {
                Side::Sell => (order.uid(), top_order.uid()),
                Side::Buy => (top_order.uid(), order.uid()),
            };

            let t = Trade {
                from,
                to,
                quantity: size_matched,
                price: top_order.price(),
                ts,
            };

            if top_order.quantity() == 0 {
                other_book.pop();
            }
        }

        if order.quantity() > 0 && !order.immediate_or_cancel() {
            book.push(order);
        }
    }
}
