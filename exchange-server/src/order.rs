use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

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
    pub created_at: u64,
    pub creator: String,
    pub size: u32,
    pub price: u32,
    pub side: Side,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub from: String,
    pub to: String,
    pub size: u32,
    pub price: u32,
    pub ts: u64,
}

impl Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}: {} @ ${}",
            self.from, self.to, self.size, self.price
        )
    }
}
