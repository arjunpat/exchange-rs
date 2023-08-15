use crate::utils;
use std::{cmp::Ordering, fmt::Display};
// use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

// this assumes all bools are is_buy
impl From<bool> for Side {
    fn from(b: bool) -> Self {
        if b {
            Self::Buy
        } else {
            Self::Sell
        }
    }
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

impl Order {
    pub fn new_limit(uid: u64, creator: String, quantity: u32, price: u32, side: Side) -> Self {
        Order {
            uid,
            created_at: utils::now(),
            creator,
            quantity,
            price,
            side,
            all_or_none: false,
            immediate_or_cancel: false,
        }
    }

    pub fn new_market(uid: u64, creator: String, quantity: u32, side: Side) -> Self {
        Order::new_limit(uid, creator, quantity, 0, side)
    }

    pub fn reduce_quantity(&mut self, amt: u32) {
        if amt > self.quantity {
            panic!("Trying to reduce order by more than quantity");
        }
        self.quantity -= amt;
    }

    pub fn price(&self) -> u32 {
        self.price
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn creator(&self) -> &str {
        &self.creator
    }

    pub fn all_or_none(&self) -> bool {
        self.all_or_none
    }

    pub fn immediate_or_cancel(&self) -> bool {
        self.immediate_or_cancel
    }

    pub fn uid(&self) -> u64 {
        self.uid
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Order {}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.side != other.side {
            panic!(
                "Comparing two orders that are not the same side {:?} and {:?}",
                self, other
            );
        }

        if self.price == other.price {
            return other.created_at.cmp(&self.created_at);
        }

        let cmp = self.price.partial_cmp(&other.price).unwrap();
        match self.side {
            Side::Buy => cmp,
            Side::Sell => cmp.reverse(),
        }
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Trade {
    pub from: u64,
    pub to: u64,
    pub quantity: u32,
    pub price: u32,
    pub ts: u64,
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
