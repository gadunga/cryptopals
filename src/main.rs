mod stats;
mod utils;
mod s1c1;
mod s1c2;
mod s1c3;
mod s1c4;
mod s1c5;
mod s1c6;
mod s1c7;

#[macro_use]
extern crate lazy_static;
extern crate hex;
extern crate base64;
extern crate crypto;

fn main() {
    let freqs = stats::init();
    println!("Start P1C1.");
    s1c1::solve();
    println!("End P1C1.");
    println!("Start P1C2.");
    s1c2::solve();
    println!("End P1C2.");
    println!("Start P1C3.");
    s1c3::solve(&freqs);
    println!("End P1C3.");
    println!("Start P1C4.");
    s1c4::solve("4.txt".to_string(), &freqs);
    println!("End P1C4.");
    println!("Start P1C5.");
    s1c5::solve();
    println!("End P1C5.");
    println!("Start P1C6.");
    s1c6::solve("6.txt".to_string(), &freqs);
    println!("End P1C6.");
    println!("Start P1C7.");
    s1c7::solve("7.txt".to_string());
    println!("End P1C7.");
}