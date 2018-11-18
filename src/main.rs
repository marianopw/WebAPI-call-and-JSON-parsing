#![allow(non_snake_case)]

extern crate reqwest;
use std::io::Read;
extern crate serde_json; // 1.0.21
use serde_json::{Value, Error};

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Foo {
    success: bool,
    result: Data,
}

#[derive(Serialize, Deserialize)]
struct Data {
    Bid: f64,
    Ask: f64,
    Last: f64,
}

fn main() -> Result<(), Error> {
     let mut response = reqwest::get("https://bittrex.com/api/v1.1/public/getticker?market=BTC-BURST")
        .expect("Failed to send request");
    let mut buf = String::new();
    response
        .read_to_string(&mut buf)
        .expect("Failed to read response");
    //println!("{}", buf);

    let foo : Foo = serde_json::from_str(&buf)?;
    println!("##########################");
    println!("BTC - BURST # BITTREX API");
    println!("##########################");
    println!("Bid: {}", foo.result.Bid);
    println!("Ask: {}", foo.result.Ask);
    println!("Last {}", foo.result.Last);

    Ok(())
}