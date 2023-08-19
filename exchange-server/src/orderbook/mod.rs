mod book;
mod chain;
mod order;
mod tests;
pub mod utils;

use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

struct Chain {
    data: BTreeSet<OrderTracker>,
    depths: HashMap<u32, u32>,
    side: Side,
}

pub struct Book {
    bids: Chain,
    asks: Chain,
    // market_data_handler: Option<Box<dyn Fn(MarketDataEvent)>>,
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
pub struct OrderTracker {
    uid: u64,
    // ordering starts at 0 and increments throughout the program
    // for internal use only
    ordering: u64,
    quantity: u32,
    price: u32,
    side: Side,
}

// used for internal record keeping
// not information to share with market
#[derive(Debug)]
pub struct Trade {
    pub from: u64,
    pub to: u64,
    pub quantity: u32,
    pub price: u32,
    pub ts: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BBO {
    pub quantity: u32,
    pub price: u32,
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
