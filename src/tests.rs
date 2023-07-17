#![cfg(test)]
use crate::{
    order::{Order, Side, Transaction},
    orderbook::OrderBook,
    utils,
};

#[test]
fn test_place_sell() {
    let mut ts: Vec<Transaction> = Vec::new();
    let mut b = OrderBook::new("AAPL");

    b.place(Order {
        created_at: 0,
        creator: "David".to_string(),
        size: 1,
        price: 100.0,
        side: Side::Sell,
    });
    b.place(Order {
        created_at: 1,
        creator: "Brian".to_string(),
        size: 5,
        price: 100.0,
        side: Side::Sell,
    });
    b.place(Order {
        created_at: 2,
        creator: "Arjun".to_string(),
        size: 20,
        price: 100.0,
        side: Side::Sell,
    });
    b.place(Order {
        created_at: 3,
        creator: "Kevin".to_string(),
        size: 4,
        price: 99.0,
        side: Side::Sell,
    });
    assert!(b.sell_orders() == 4);
    assert!(b.current_price() == 0.0);

    b.place(Order {
        created_at: 4,
        creator: "Andrew".to_string(),
        size: 2,
        price: 99.0,
        side: Side::Buy,
    });
    assert!(b.current_price() == 99.0);
    ts.push(Transaction {
        from: "Kevin".to_string(),
        to: "Andrew".to_string(),
        security: "AAPL".to_string(),
        size: 2,
        price: 99.0,
        ts: utils::now(),
    });

    b.place(Order {
        created_at: 5,
        creator: "Bob".to_string(),
        size: 4,
        price: 100.0,
        side: Side::Buy,
    });
    assert!(b.current_price() == 100.0);
    ts.push(Transaction {
        from: "Kevin".to_string(),
        to: "Bob".to_string(),
        security: "AAPL".to_string(),
        size: 2,
        price: 99.0,
        ts: utils::now(),
    });
    ts.push(Transaction {
        from: "David".to_string(),
        to: "Bob".to_string(),
        security: "AAPL".to_string(),
        size: 1,
        price: 100.0,
        ts: utils::now(),
    });
    ts.push(Transaction {
        from: "Brian".to_string(),
        to: "Bob".to_string(),
        security: "AAPL".to_string(),
        size: 1,
        price: 100.0,
        ts: utils::now(),
    });

    b.place(Order {
        created_at: 6,
        creator: "Jake".to_string(),
        size: 5,
        price: 103.0,
        side: Side::Buy,
    });
    assert!(b.current_price() == 100.0);
    ts.push(Transaction {
        from: "Brian".to_string(),
        to: "Jake".to_string(),
        security: "AAPL".to_string(),
        size: 4,
        price: 100.0,
        ts: utils::now(),
    });
    ts.push(Transaction {
        from: "Arjun".to_string(),
        to: "Jake".to_string(),
        security: "AAPL".to_string(),
        size: 1,
        price: 100.0,
        ts: utils::now(),
    });
    for i in 0..b.transactions.len() {
        assert!(b.transactions[i].to_string() == ts[i].to_string());
    }
}

#[test]
fn test_place_buy() {
    let mut ts: Vec<Transaction> = Vec::new();
    let mut b = OrderBook::new("AAPL");

    b.place(Order {
        created_at: 0,
        creator: "David".to_string(),
        size: 1,
        price: 100.0,
        side: Side::Buy,
    });
    b.place(Order {
        created_at: 1,
        creator: "Brian".to_string(),
        size: 5,
        price: 100.0,
        side: Side::Buy,
    });
    b.place(Order {
        created_at: 2,
        creator: "Arjun".to_string(),
        size: 20,
        price: 100.0,
        side: Side::Buy,
    });
    b.place(Order {
        created_at: 3,
        creator: "Kevin".to_string(),
        size: 4,
        price: 101.0,
        side: Side::Buy,
    });
    assert!(b.buy_orders() == 4);
    assert!(b.current_price() == 0.0);

    b.place(Order {
        created_at: 4,
        creator: "Andrew".to_string(),
        size: 2,
        price: 101.0,
        side: Side::Sell,
    });
    assert!(b.current_price() == 101.0);
    ts.push(Transaction {
        from: "Andrew".to_string(),
        to: "Kevin".to_string(),
        security: "AAPL".to_string(),
        size: 2,
        price: 101.0,
        ts: utils::now(),
    });

    b.place(Order {
        created_at: 5,
        creator: "Bob".to_string(),
        size: 4,
        price: 100.0,
        side: Side::Sell,
    });
    assert!(b.current_price() == 100.0);
    ts.push(Transaction {
        from: "Bob".to_string(),
        to: "Kevin".to_string(),
        security: "AAPL".to_string(),
        size: 2,
        price: 101.0,
        ts: utils::now(),
    });
    ts.push(Transaction {
        from: "Bob".to_string(),
        to: "David".to_string(),
        security: "AAPL".to_string(),
        size: 1,
        price: 100.0,
        ts: utils::now(),
    });
    ts.push(Transaction {
        from: "Bob".to_string(),
        to: "Brian".to_string(),
        security: "AAPL".to_string(),
        size: 1,
        price: 100.0,
        ts: utils::now(),
    });

    b.place(Order {
        created_at: 6,
        creator: "Jake".to_string(),
        size: 5,
        price: 98.0,
        side: Side::Sell,
    });
    ts.push(Transaction {
        from: "Jake".to_string(),
        to: "Brian".to_string(),
        security: "AAPL".to_string(),
        size: 4,
        price: 100.0,
        ts: utils::now(),
    });
    ts.push(Transaction {
        from: "Jake".to_string(),
        to: "Arjun".to_string(),
        security: "AAPL".to_string(),
        size: 1,
        price: 100.0,
        ts: utils::now(),
    });
    assert!(b.current_price() == 100.0);
    for i in 0..b.transactions.len() {
        assert!(b.transactions[i].to_string() == ts[i].to_string());
    }
}
