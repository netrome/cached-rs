extern crate lru_macro;

#[lru_macro::hola]
fn derp(){
    println!("Derpface");
}

#[lru_macro::wip]
fn herp(myparam: i64) -> i64{
    return myparam * 2
}

use std::collections::HashMap;
fn cached_herp(myparam: i64) -> i64{

}

fn main() {
    println!("Hello, world!");
    hello();
}
