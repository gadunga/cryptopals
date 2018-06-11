extern crate hex;

use std::collections::HashMap;
use std::f32;
use utils;

pub fn solve(freqs: &HashMap<u8, f32>) {
    let mut message_score = f32::MAX;
    let mut message = String::from("");
    let mut xor_char: char = 'a';

    let bytes_a = match hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736") {
        Ok(n) => n,
        Err(e) => panic!(e)
    };

    for c in b' ' ..= b'~' {
        let res_tuple = utils::xor_and_score(&bytes_a, &c, freqs);
        match String::from_utf8(res_tuple.1) {
            Ok(n) => {
                if res_tuple.0 < message_score {
                    message_score = res_tuple.0;
                    message = n;
                    xor_char = c as char;
                }  
            },
            Err(_e) => continue
        };
    }

    println!("{} - {} - {}", xor_char, message_score, message);
}