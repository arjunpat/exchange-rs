use std::{
    cell::Cell,
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

use super::{utils, Chain, Order, OrderTracker, Side};

impl OrderTracker {
    pub fn from_order(ord: &Order) -> Self {
        Self {
            uid: ord.uid(),
            created_at: utils::now(),
            quantity: ord.quantity(),
            side: ord.side(),
            price: ord.price(),
        }
    }

    pub fn into_bounds(price: u32, side: Side) -> (Self, Self) {
        let first = Self {
            uid: 0,
            price,
            created_at: 0,
            quantity: 0,
            side,
        };
        let second = Self {
            uid: 0,
            price,
            created_at: std::u64::MAX,
            quantity: 0,
            side,
        };
        (first, second)
    }
}

impl PartialEq for OrderTracker {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for OrderTracker {}

impl Ord for OrderTracker {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.side != other.side {
            panic!(
                "Comparing two orders that are not the same side {:?} and {:?}",
                self, other
            );
        }

        if self.price == other.price {
            return self.created_at.cmp(&other.created_at);
        }

        let cmp = self.price.cmp(&other.price);
        match self.side {
            Side::Buy => cmp.reverse(),
            Side::Sell => cmp,
        }
    }
}

impl PartialOrd for OrderTracker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Chain {
    pub fn new(side: Side) -> Self {
        Self {
            data: BTreeSet::new(),
            depths: HashMap::new(),
            side,
            bbo: Cell::new(None),
            bbo_handler: None,
        }
    }

    pub fn set_bbo_handler(&mut self, f: fn(Side, Option<u32>)) {
        self.bbo_handler = Some(f);
    }

    pub fn bbo(&self) -> Option<u32> {
        self.bbo.get()
    }

    fn update_bbo(&self) {
        let new_bbo = match self.data.first() {
            Some(ot) => Some(ot.price),
            None => None,
        };

        if self.bbo.replace(new_bbo) != new_bbo {
            if let Some(handler) = self.bbo_handler {
                handler(self.side, self.bbo.get());
            }
        }
    }

    pub fn first(&self) -> Option<&OrderTracker> {
        self.data.first()
    }

    pub fn pop_first(&mut self) -> Option<OrderTracker> {
        let ot = self.data.pop_first()?;

        // update state
        *self.depths.get_mut(&ot.price)? -= ot.quantity;

        self.update_bbo();

        Some(ot)
    }

    pub fn insert(&mut self, ot: OrderTracker) {
        // update state
        if !self.depths.contains_key(&ot.price) {
            self.depths.insert(ot.price, 0);
        }
        *self.depths.get_mut(&ot.price).unwrap() += ot.quantity;

        self.data.insert(ot);

        // update state
        self.update_bbo();
    }

    fn find_and_clone(&self, price: u32, uid: u64) -> Option<OrderTracker> {
        let (from, to) = OrderTracker::into_bounds(price, self.side);

        let candidates: Vec<&OrderTracker> =
            self.data.range(from..to).filter(|e| e.uid == uid).collect();

        if candidates.len() != 1 {
            return None;
        }

        let ot_ref = *candidates.get(0).unwrap();
        // is there a way to do this without cloning?
        Some(ot_ref.clone())
    }

    pub fn remove(&mut self, price: u32, uid: u64) -> Option<OrderTracker> {
        let ot_ref = self.find_and_clone(price, uid)?;

        let ot_removed = self.data.take(&ot_ref)?;

        // update state
        *self.depths.get_mut(&ot_removed.price).unwrap() -= ot_removed.quantity;
        self.update_bbo();

        Some(ot_removed)
    }
}
