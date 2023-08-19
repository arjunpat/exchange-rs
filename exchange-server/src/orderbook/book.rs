use super::{utils, Book, Chain, Order, OrderTracker, Side, Trade, BBO};
use std::collections::HashMap;

impl Book {
    pub fn new() -> Self {
        Self {
            bids: Chain::new(Side::Buy),
            asks: Chain::new(Side::Sell),
        }
    }

    pub fn bbo(&self) -> (Option<BBO>, Option<BBO>) {
        (self._bbo(&self.bids), self._bbo(&self.asks))
    }

    fn _bbo(&self, chain: &Chain) -> Option<BBO> {
        let ot = chain.first()?;
        Some(BBO {
            price: ot.price,
            quantity: *chain.depths.get(&ot.price)?,
        })
    }

    pub fn get_depths(&self) -> (HashMap<u32, u32>, HashMap<u32, u32>) {
        (self.bids.depths.clone(), self.asks.depths.clone())
    }

    pub fn cancel_order(&mut self, side: Side, price: u32, uid: u64) -> Option<OrderTracker> {
        let book = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        book.remove(price, uid)
    }

    // all market orders are assumed to be immediate or cancel
    pub fn place_market(&mut self, ord: &Order) -> Vec<Trade> {
        let mut order = OrderTracker::from_order(ord);
        let other_book = match order.side {
            Side::Buy => &mut self.asks,
            Side::Sell => &mut self.bids,
        };

        let mut trades = Vec::new();

        assert!(!ord.all_or_none());

        // all or none not yet supported
        while let Some(mut top_order) = other_book.pop_first() {
            let size_matched = top_order.quantity.min(order.quantity);
            top_order.quantity -= size_matched;
            order.quantity -= size_matched;

            let (from, to) = match order.side {
                Side::Sell => (order.uid, top_order.uid),
                Side::Buy => (top_order.uid, order.uid),
            };

            trades.push(Trade {
                from,
                to,
                price: top_order.price,
                ts: utils::now(),
                quantity: size_matched,
            });

            if top_order.quantity > 0 {
                other_book.insert(top_order);
            }
            if order.quantity == 0 {
                break;
            }
        }

        trades
    }

    pub fn place_limit(&mut self, ord: &Order) -> Vec<Trade> {
        let mut order = OrderTracker::from_order(ord);

        let (book, other_book) = match order.side {
            Side::Buy => (&mut self.bids, &mut self.asks),
            Side::Sell => (&mut self.asks, &mut self.bids),
        };

        let mut trades = Vec::new();

        // all_or_none is not yet supported for limit orders
        assert!(!ord.all_or_none());

        loop {
            if order.quantity == 0 {
                break;
            }

            let top_order = other_book.first();
            if top_order.is_none() {
                break;
            }
            let top_order = top_order.unwrap();

            let liquidity_unavailable = match order.side {
                Side::Buy => top_order.price > order.price,
                Side::Sell => top_order.price < order.price,
            };

            if liquidity_unavailable {
                println!("Liquidity not available {:?}", top_order);
                break;
            }

            let size_matched = top_order.quantity.min(order.quantity);

            // consider using RefCell to avoid removing from BTreeSet
            let mut top_order = other_book.pop_first().unwrap();
            top_order.quantity -= size_matched;
            order.quantity -= size_matched;

            let (from, to) = match order.side {
                Side::Sell => (order.uid, top_order.uid),
                Side::Buy => (top_order.uid, order.uid),
            };

            trades.push(Trade {
                from,
                to,
                quantity: size_matched,
                price: top_order.price,
                ts: utils::now(),
            });

            if top_order.quantity > 0 {
                other_book.insert(top_order);
            }
        }

        if order.quantity > 0 && !ord.immediate_or_cancel() {
            book.insert(order);
        }

        trades
    }
}
