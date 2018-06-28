mod c1;
mod c2;
mod c3;
mod c4;
mod c5;
mod c6;
mod c7;
mod c8;

use stats;

pub fn solve() {
    let freqs = stats::init();
    println!("Start P1C1.");
    c1::solve();
    println!("End P1C1.");
    println!("Start P1C2.");
    c2::solve();
    println!("End P1C2.");
    println!("Start P1C3.");
    c3::solve(&freqs);
    println!("End P1C3.");
    println!("Start P1C4.");
    c4::solve("4.txt".to_string(), &freqs);
    println!("End P1C4.");
    println!("Start P1C5.");
    c5::solve();
    println!("End P1C5.");
    println!("Start P1C6.");
    c6::solve("6.txt".to_string(), &freqs);
    println!("End P1C6.");
    println!("Start P1C7.");
    c7::solve("7.txt".to_string());
    println!("End P1C7.");
    println!("Start P1C8.");
    c8::solve("8.txt".to_string());
    println!("End P1C8.");
}