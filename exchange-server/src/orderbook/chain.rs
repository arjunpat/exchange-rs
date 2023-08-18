use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    sync::atomic,
};

use super::{Chain, Order, OrderTracker, Side};

static ORDER_TRACKER_COUNTER: atomic::AtomicU64 = atomic::AtomicU64::new(0);

impl OrderTracker {
    pub fn from_order(ord: &Order) -> Self {
        let ordering = ORDER_TRACKER_COUNTER.fetch_add(1, atomic::Ordering::Relaxed);
        Self {
            uid: ord.uid(),
            ordering,
            quantity: ord.quantity(),
            side: ord.side(),
            price: ord.price(),
        }
    }

    pub fn into_bounds(price: u32, side: Side) -> (Self, Self) {
        let first = Self {
            uid: 0,
            price,
            ordering: 0,
            quantity: 0,
            side,
        };
        let second = Self {
            uid: 0,
            price,
            ordering: std::u64::MAX,
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
            return self.ordering.cmp(&other.ordering);
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
        }
    }

    pub fn first(&self) -> Option<&OrderTracker> {
        self.data.first()
    }

    pub fn pop_first(&mut self) -> Option<OrderTracker> {
        let ot = self.data.pop_first()?;

        // update state
        *self.depths.get_mut(&ot.price)? -= ot.quantity;

        Some(ot)
    }

    pub fn insert(&mut self, ot: OrderTracker) {
        // update state
        if !self.depths.contains_key(&ot.price) {
            self.depths.insert(ot.price, 0);
        }
        *self.depths.get_mut(&ot.price).unwrap() += ot.quantity;

        if self.data.contains(&ot) {
            println!("ALREADY FOUND: {:?} {:?}", ot, self.data.get(&ot));
            assert!(false);
        }
        assert!(!self.data.contains(&ot));
        self.data.insert(ot);
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

        Some(ot_removed)
    }
}
