#![allow(dead_code)]

mod order;
mod orderbook;
mod utils;
use order::{Order, Side};

fn main() {
    println!("Hello, world!");

    let ord = Order::new("arjunpat", 10, 12.55, Side::Buy);

    println!("{:#?}", ord);
}
