use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Order {
    pub created_at: u64,
    pub creator: String,
    pub size: i64,
    pub price: f64,
    pub side: Side,
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
            return other.created_at.cmp(&self.created_at);
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
    pub from: String,
    pub to: String,
    pub security: String,
    pub size: i64,
    pub price: f64,
    pub ts: u64,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}: {} of {} @ ${}",
            self.from, self.to, self.size, self.security, self.price
        )
    }
}
