use super::{Book, Chain, MarketDataEvent, Order, OrderTracker, Side, Trade, BBO};
use std::collections::HashMap;

impl Book {
    pub fn new() -> Self {
        let bids = Chain::new(Side::Buy);
        let asks = Chain::new(Side::Sell);

        Self {
            bids,
            asks,
            market_data_handler: None,
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

    pub fn set_market_data_handler(&mut self, f: impl Fn(MarketDataEvent) + 'static) {
        self.market_data_handler = Some(Box::new(f));
    }

    pub fn get_depths(&self) -> (HashMap<u32, u32>, HashMap<u32, u32>) {
        (self.bids.depths.clone(), self.asks.depths.clone())
    }

    pub fn cancel_order(&mut self, ord: &Order) {
        let book = match ord.side() {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        book.remove(ord.price(), ord.uid());
    }

    // all market orders are assumed to be immediate or cancel
    pub fn place_market(&mut self, ord: &Order) {
        let mut order = OrderTracker::from_order(ord);
        let (book, other_book) = match order.side {
            Side::Buy => (&mut self.bids, &mut self.asks),
            Side::Sell => (&mut self.asks, &mut self.bids),
        };

        let mut trades = Vec::new();

        // all or none not yet supported
        while let Some(mut top_order) = other_book.pop_first() {
            let size_matched = top_order.quantity.min(order.quantity);
            top_order.quantity -= size_matched;
            order.quantity -= size_matched;

            let (from, to) = match order.side {
                Side::Sell => (order.uid, top_order.uid),
                Side::Buy => (top_order.uid, order.uid),
            };

            if self.market_data_handler.is_some() {
                trades.push(Trade {
                    from,
                    to,
                    price: top_order.price,
                    ts: order.created_at,
                    quantity: size_matched,
                });
            }

            if top_order.quantity > 0 {
                other_book.insert(top_order);
            }
            if order.quantity == 0 {
                break;
            }
        }

        if trades.len() > 0 {
            if let Some(ref handler) = self.market_data_handler {
                handler(MarketDataEvent::Trades(trades));
            }
        }
    }

    pub fn place_limit(&mut self, ord: &Order) {
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

            if self.market_data_handler.is_some() {
                trades.push(Trade {
                    from,
                    to,
                    quantity: size_matched,
                    price: top_order.price,
                    ts: order.created_at,
                });
            }

            if top_order.quantity > 0 {
                other_book.insert(top_order);
            }
        }

        if order.quantity > 0 && !ord.immediate_or_cancel() {
            book.insert(order);
        }

        if trades.len() > 0 {
            if let Some(ref handler) = self.market_data_handler {
                handler(MarketDataEvent::Trades(trades));
            }
        }
    }
}
