#![cfg(test)]
use super::*;

fn trade_equal(a: &Trade, b: &Trade) -> bool {
    return a.from == b.from && a.to == b.to && a.quantity == b.quantity && a.price == b.price;
}

fn assert_trades_equal(a: &Vec<Trade>, b: &Vec<Trade>) {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        assert!(trade_equal(&a[i], &b[i]));
    }
}

#[test]
fn test_place_limit_buy() {
    let mut b = Book::new();
    let mut confirmed_ts = Vec::new();

    b.place_limit(&Order::new_limit(0, "David", 1, 100, Side::Buy));
    b.place_limit(&Order::new_limit(1, "Brain", 5, 100, Side::Buy));
    b.place_limit(&Order::new_limit(2, "Arjun", 20, 100, Side::Buy));
    b.place_limit(&Order::new_limit(3, "kevin", 4, 101, Side::Buy));
    assert_eq!(*b.get_depths().0.get(&100).unwrap(), 26);
    assert_eq!(
        b.bbo(),
        (
            Some(BBO {
                price: 101,
                quantity: 4,
            }),
            None,
        )
    );

    let trades = b.place_limit(&Order::new_limit(4, "Andrew", 2, 101, Side::Sell));
    assert_eq!(*b.get_depths().0.get(&101).unwrap(), 2);
    confirmed_ts.push(Trade {
        from: 4,
        to: 3,
        price: 101,
        ts: 0,
        quantity: 2,
    });
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();

    let trades = b.place_limit(&Order::new_limit(5, "Bob", 4, 100, Side::Sell));
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
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();

    let trades = b.place_limit(&Order::new_limit(6, "Jake", 5, 98, Side::Sell));
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
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();

    let trades = b.place_limit(&Order::new_limit(7, "Arjun", 6, 101, Side::Buy));
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();

    let trades = b.place_market(&Order::new_market(8, "Bill", 20, Side::Sell));
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
    assert_eq!(
        b.bbo(),
        (
            Some(BBO {
                price: 100,
                quantity: 5
            }),
            None
        )
    );
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();

    let trades = b.place_market(&Order::new_market(9, "Bill", 20, Side::Sell));
    confirmed_ts.push(Trade {
        from: 9,
        to: 2,
        quantity: 5,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().0.get(&100).unwrap(), 0);
    assert_eq!(b.bbo(), (None, None));
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();
}

#[test]
fn test_place_limit_sell() {
    let mut b = Book::new();
    let mut confirmed_ts = Vec::new();

    b.place_limit(&Order::new_limit(0, "David", 1, 100, Side::Sell));
    b.place_limit(&Order::new_limit(1, "Brain", 5, 100, Side::Sell));
    b.place_limit(&Order::new_limit(2, "Arjun", 20, 100, Side::Sell));
    b.place_limit(&Order::new_limit(3, "kevin", 4, 99, Side::Sell));
    assert_eq!(*b.get_depths().1.get(&100).unwrap(), 26);
    assert_eq!(
        b.bbo(),
        (
            None,
            Some(BBO {
                price: 99,
                quantity: 4,
            }),
        )
    );

    let trades = b.place_limit(&Order::new_limit(4, "Andrew", 2, 99, Side::Buy));
    assert_eq!(*b.get_depths().1.get(&99).unwrap(), 2);
    confirmed_ts.push(Trade {
        from: 3,
        to: 4,
        price: 99,
        ts: 0,
        quantity: 2,
    });
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();

    let trades = b.place_limit(&Order::new_limit(5, "Bob", 4, 100, Side::Buy));
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
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();

    let trades = b.place_limit(&Order::new_limit(6, "Jake", 5, 103, Side::Buy));
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
    println!("Book: {:?}", b.asks.data);
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();
    println!("Sell depths: {:?}", b.get_depths().1);
    assert_eq!(*b.get_depths().1.get(&100).unwrap(), 19);

    let trades = b.place_limit(&Order::new_limit(7, "Arjun", 6, 99, Side::Sell));
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();

    let trades = b.place_market(&Order::new_market(8, "Bill", 20, Side::Buy));
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
    assert_eq!(
        b.bbo(),
        (
            None,
            Some(BBO {
                price: 100,
                quantity: 5
            })
        )
    );
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();

    let trades = b.place_market(&Order::new_market(9, "Bill", 20, Side::Buy));
    confirmed_ts.push(Trade {
        from: 2,
        to: 9,
        quantity: 5,
        price: 100,
        ts: 0,
    });
    assert_eq!(*b.get_depths().1.get(&100).unwrap(), 0);
    assert_eq!(b.bbo(), (None, None));
    assert_trades_equal(&trades, &confirmed_ts);
    confirmed_ts.clear();
}
