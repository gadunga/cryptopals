mod stats;
mod utils;
mod s1c1;
mod s1c2;
mod s1c3;
mod s1c4;

#[macro_use]
extern crate lazy_static;
extern crate hex;
extern crate base64;

fn main() {
    let freqs = stats::init();
    s1c1::solve();
    s1c2::solve();
    s1c3::solve(&freqs);
    s1c4::solve("4.txt".to_string(), &freqs);
}