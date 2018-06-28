#![allow(dead_code)]

extern crate hex;

use std::collections::HashMap;
use std::f32;
use std::fs::File;
use std::io::{
    BufRead,
    BufReader,
};
use utils;

pub fn solve(path: String, freqs: &HashMap<u8, f32>) {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut pairs: Vec<(f32, String)> = Vec::new();

    for l in reader.lines() {
        let temp_line = l.unwrap();
        let line_bytes = match hex::decode(temp_line) {
            Ok(n) => n,
            Err(e) => panic!(e)
        };

        for c in b' ' ..= b'~' {
            let res_tuple = utils::xor_and_score(&line_bytes, &c, freqs, 0.8);
            match String::from_utf8(res_tuple.1) {
                Ok(n) => {
                    if n.len() > 1 && n.is_ascii() {
                        pairs.push((res_tuple.0, n));
                    } 
                },
                Err(_e) => continue
            };
        }
    }

    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    pairs.iter().take(1).for_each(|(a, b)| println!("{}: {}", a, b));
}