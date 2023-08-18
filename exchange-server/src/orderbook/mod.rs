mod book;
mod chain;
mod order;
mod tests;
pub mod utils;

use std::{
    cell::Cell,
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

struct Chain {
    data: BTreeSet<OrderTracker>,
    depths: HashMap<u32, u32>,
    side: Side,
    bbo: Cell<Option<u32>>,
    bbo_handler: Option<fn(Side, Option<u32>)>,
}

pub struct Book {
    bids: Chain,
    asks: Chain,
    market_data_handler: Option<Box<dyn Fn(MarketDataEvent)>>,
}

#[derive(Debug)]
pub struct Order {
    uid: u64,
    created_at: u64,
    creator: String,
    quantity: u32,
    price: u32,
    side: Side,
    all_or_none: bool,
    immediate_or_cancel: bool,
}

#[derive(Debug, Clone)]
struct OrderTracker {
    uid: u64,
    // created_at is the time when this object is created in the order book
    // in other words, when we actually receive the order
    created_at: u64,
    quantity: u32,
    price: u32,
    side: Side,
}

#[derive(Debug)]
pub struct Trade {
    pub from: u64,
    pub to: u64,
    pub quantity: u32,
    pub price: u32,
    pub ts: u64,
}

#[derive(Debug)]
pub struct BBO {
    price: u32,
    quantity: u32,
}

#[derive(Debug)]
pub enum MarketDataEvent {
    Trades(Vec<Trade>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub fn is_buy(&self) -> bool {
        self == &Side::Buy
    }

    pub fn from_is_buy(is_buy: bool) -> Self {
        match is_buy {
            true => Self::Buy,
            false => Self::Sell,
        }
    }
}

impl Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}: {} @ ${}",
            self.from, self.to, self.quantity, self.price
        )
    }
}
