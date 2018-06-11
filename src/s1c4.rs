extern crate hex;

use std::collections::HashMap;
use std::f32;
use std::fs::File;
use std::io::{
    BufRead,
    BufReader,
};
use utils;

// Answer is: 7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
pub fn solve(path: String, freqs: &HashMap<u8, f32>) {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut message_score = f32::MAX;
    let mut message: String;
    let mut line: String;
    let mut xor_char: char;

    for l in reader.lines() {
        let temp_line = l.unwrap();
        let line_bytes = match hex::decode(temp_line.clone()) {
            Ok(n) => n,
            Err(e) => panic!(e)
        };

        for c in b' ' ..= b'~' {
            let res_tuple = utils::xor_and_score(&line_bytes, &c, freqs);
            match String::from_utf8(res_tuple.1) {
                Ok(n) => {
                    if res_tuple.0 < message_score {
                        message_score = res_tuple.0;
                        message = n;
                        xor_char = c as char;
                        line = temp_line.clone();

                        if message.len() > 1 && message.is_ascii() {
                            println!("{} - {} - {} - {}", xor_char, message_score, line, message); 
                        }
                    }  
                },
                Err(_e) => continue
            };
        }
    }
}