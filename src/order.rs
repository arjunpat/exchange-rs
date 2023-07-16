use std::{cmp::Ordering, fmt::Display};

use crate::utils;

#[derive(Debug, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Order {
    created_at: u64,
    creator: String,
    size: i64,
    price: f64,
    side: Side,
}

impl Order {
    pub fn new(creator: &str, size: i64, price: f64, side: Side) -> Order {
        return Order {
            creator: creator.to_owned(),
            created_at: utils::now(),
            side,
            size,
            price,
        };
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl Eq for Order {}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.side == other.side {
            if self.price != other.price {
                let cmp = self.price.partial_cmp(&other.price).unwrap();
                if self.side == Side::Buy {
                    return cmp;
                } else {
                    return cmp.reverse();
                }
            }
            return self.created_at.cmp(&other.created_at);
        }
        panic!(
            "Comparing two orders that are not the same side {:?} and {:?}",
            self, other
        );
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Transaction {
    from: String,
    to: String,
    security: String,
    size: i64,
    pub price: f64,
    ts: u64,
}

impl Transaction {
    pub fn new(from: &str, to: &str, security: &str, size: i64, price: f64) -> Transaction {
        Transaction {
            from: from.to_owned(),
            to: to.to_owned(),
            security: security.to_owned(),
            size,
            price,
            ts: utils::now(),
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}: {} of {} @ ${} at time {}",
            self.from, self.to, self.size, self.security, self.price, self.ts
        )
    }
}
