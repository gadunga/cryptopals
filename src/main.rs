#[macro_use]
extern crate lazy_static;
extern crate hex;
extern crate base64;
extern crate crypto;

mod stats;
mod utils;
mod set1;

fn main() {
    set1::solve();
}