#![cfg(test)]
use super::*;
use std::{cell::RefCell, rc::Rc};

fn trades_equal(a: &Trade, b: &Trade) -> bool {
    return a.from == b.from && a.to == b.to && a.quantity == b.quantity && a.price == b.price;
}

#[test]
fn test_place_limit_buy() {
    let mut b = Book::new();
    let mut confirmed_ts = Vec::new();
    let ts: Rc<RefCell<Vec<Trade>>> = Rc::new(RefCell::new(Vec::new()));

    let ts1 = ts.clone();
    b.set_market_data_handler(move |d: MarketDataEvent| {
        let mut vec = ts1.borrow_mut();

        match d {
            MarketDataEvent::Trades(trades) => {
                for each in trades {
                    vec.push(each);
                }
            }
            _ => {}
        }
    });

    b.place_limit(&Order::new_limit(0, "David", 1, 100, Side::Buy));
    b.place_limit(&Order::new_limit(1, "Brain", 5, 100, Side::Buy));
    b.place_limit(&Order::new_limit(2, "Arjun", 20, 100, Side::Buy));
    b.place_limit(&Order::new_limit(3, "kevin", 4, 101, Side::Buy));
    assert_eq!(*b.get_depths().0.get(&100).unwrap(), 26);

    b.place_limit(&Order::new_limit(4, "Andrew", 2, 101, Side::Sell));
    assert_eq!(*b.get_depths().0.get(&101).unwrap(), 2);
    confirmed_ts.push(Trade {
        from: 4,
        to: 3,
        price: 101,
        ts: 0,
        quantity: 2,
    });

    b.place_limit(&Order::new_limit(5, "Bob", 4, 100, Side::Sell));
    confirmed_ts.push(Trade {
        from: 5,
        to: 3,
        quantity: 2,
        price: 101,
        ts: 0,
    });
    confirmed_ts.push(Trade {
        from: 5,
        to: 0,
        quantity: 1,
        price: 100,
        ts: 0,
    });
    confirmed_ts.push(Trade {
        from: 5,
        to: 1,
        quantity: 1,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().0.get(&100).unwrap(), 24);

    b.place_limit(&Order::new_limit(6, "Jake", 5, 98, Side::Sell));
    confirmed_ts.push(Trade {
        from: 6,
        to: 1,
        quantity: 4,
        price: 100,
        ts: 0,
    });
    confirmed_ts.push(Trade {
        from: 6,
        to: 2,
        quantity: 1,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().0.get(&100).unwrap(), 19);

    b.place_limit(&Order::new_limit(7, "Arjun", 6, 101, Side::Buy));
    b.place_market(&Order::new_market(8, "Bill", 20, Side::Sell));
    confirmed_ts.push(Trade {
        from: 8,
        to: 7,
        quantity: 6,
        price: 101,
        ts: 0,
    });
    confirmed_ts.push(Trade {
        from: 8,
        to: 2,
        quantity: 14,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().0.get(&100).unwrap(), 5);

    b.place_market(&Order::new_market(9, "Bill", 20, Side::Sell));
    confirmed_ts.push(Trade {
        from: 9,
        to: 2,
        quantity: 5,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().0.get(&100).unwrap(), 0);

    assert_eq!(ts.borrow().len(), confirmed_ts.len());
    for i in 0..confirmed_ts.len() {
        assert!(trades_equal(&ts.borrow()[i], &confirmed_ts[i]));
    }
}

#[test]
fn test_place_limit_sell() {
    let mut b = Book::new();
    let mut confirmed_ts = Vec::new();
    let ts: Rc<RefCell<Vec<Trade>>> = Rc::new(RefCell::new(Vec::new()));

    let ts1 = ts.clone();
    b.set_market_data_handler(move |d: MarketDataEvent| {
        let mut vec = ts1.borrow_mut();
        match d {
            MarketDataEvent::Trades(trades) => {
                for each in trades {
                    vec.push(each);
                }
            }
            _ => {}
        }
    });

    b.place_limit(&Order::new_limit(0, "David", 1, 100, Side::Sell));
    b.place_limit(&Order::new_limit(1, "Brain", 5, 100, Side::Sell));
    b.place_limit(&Order::new_limit(2, "Arjun", 20, 100, Side::Sell));
    b.place_limit(&Order::new_limit(3, "kevin", 4, 99, Side::Sell));
    assert_eq!(*b.get_depths().1.get(&100).unwrap(), 26);

    b.place_limit(&Order::new_limit(4, "Andrew", 2, 99, Side::Buy));
    assert_eq!(*b.get_depths().1.get(&99).unwrap(), 2);
    confirmed_ts.push(Trade {
        from: 3,
        to: 4,
        price: 99,
        ts: 0,
        quantity: 2,
    });

    b.place_limit(&Order::new_limit(5, "Bob", 4, 100, Side::Buy));
    confirmed_ts.push(Trade {
        from: 3,
        to: 5,
        quantity: 2,
        price: 99,
        ts: 0,
    });
    confirmed_ts.push(Trade {
        from: 0,
        to: 5,
        quantity: 1,
        price: 100,
        ts: 0,
    });
    confirmed_ts.push(Trade {
        from: 1,
        to: 5,
        quantity: 1,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().1.get(&100).unwrap(), 24);

    b.place_limit(&Order::new_limit(6, "Jake", 5, 103, Side::Buy));
    confirmed_ts.push(Trade {
        from: 1,
        to: 6,
        quantity: 4,
        price: 100,
        ts: 0,
    });
    confirmed_ts.push(Trade {
        from: 2,
        to: 6,
        quantity: 1,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().1.get(&100).unwrap(), 19);

    b.place_limit(&Order::new_limit(7, "Arjun", 6, 99, Side::Sell));
    b.place_market(&Order::new_market(8, "Bill", 20, Side::Buy));
    confirmed_ts.push(Trade {
        from: 7,
        to: 8,
        quantity: 6,
        price: 99,
        ts: 0,
    });
    confirmed_ts.push(Trade {
        from: 2,
        to: 8,
        quantity: 14,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().1.get(&100).unwrap(), 5);

    b.place_market(&Order::new_market(9, "Bill", 20, Side::Buy));
    confirmed_ts.push(Trade {
        from: 2,
        to: 9,
        quantity: 5,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().1.get(&100).unwrap(), 0);

    assert_eq!(ts.borrow().len(), confirmed_ts.len());
    for i in 0..confirmed_ts.len() {
        assert!(trades_equal(&ts.borrow()[i], &confirmed_ts[i]));
    }
}
