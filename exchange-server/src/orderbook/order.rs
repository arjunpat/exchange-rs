use super::{utils, Order, Side};

impl Order {
    pub fn new_limit(uid: u64, creator: &str, quantity: u32, price: u32, side: Side) -> Self {
        Self {
            uid,
            created_at: utils::now(),
            creator: creator.to_owned(),
            quantity,
            price,
            side,
            all_or_none: false,
            immediate_or_cancel: false,
        }
    }

    pub fn new_market(uid: u64, creator: &str, quantity: u32, side: Side) -> Self {
        Order::new_limit(uid, creator, quantity, 0, side)
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
