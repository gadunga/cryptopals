mod stats;
#[macro_use]
extern crate lazy_static;
extern crate hex;
extern crate base64;

use std::collections::HashMap;
use std::fs::File;
use std::io::{
    BufRead,
    BufReader,
};

fn main() {
    let freqs = stats::init();
    problem_1();
    problem_2();
    problem_3(&freqs);
    problem_4("4.txt".to_string(), &freqs);
}

fn problem_1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = match hex::decode(input) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };
    
    println!("{}", base64::encode(&bytes));
    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", base64::encode(&bytes));
}

fn problem_2() {
    let res_2 = hex_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965");
    println!("{}", res_2);
    assert_eq!("746865206b696420646f6e277420706c6179", res_2);
}

fn problem_3(freqs: &HashMap<char, f32>) {
    let mut message_score = std::f32::MAX;
    let mut message = String::from("");
    let mut xor_char: char = 'a';

    for c in b' ' ..= b'~' {
        match decode_and_score("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736", &(c as char), freqs) {
            Some(n) => {
                if n.0 < message_score {
                    message_score = n.0;
                    message = n.1;
                    xor_char = c as char;
                }
            },
            None => ()
        }    
    }

    println!("{} - {} - {}", xor_char, message_score, message);
}

// Answer is: 7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
fn problem_4(path: String, freqs: &HashMap<char, f32>) {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    for l in reader.lines() {
        let mut message_score = std::f32::MAX;
        let mut message: String;
        let mut line: String;
        let mut xor_char: char;

        let temp_line = l.unwrap();
        for c in b' ' ..= b'~' {
            match decode_and_score(&temp_line, &(c as char), freqs) {
                Some(n) => {
                    if n.0 < message_score {
                        message_score = n.0;
                        message = n.1;
                        xor_char = c as char;
                        line = temp_line.clone();

                        if message.len() > 1 && message.is_ascii() {
                            println!("{} - {} - {} - {}", xor_char, message_score, line, message); 
                        }
                    }
                },
                None => ()
            }
        }
    }
}

fn decode_and_score(in_str: &str, c: &char, freqs: &HashMap<char, f32>) -> Option<(f32, String)> {
    match hex_xor_single(in_str, c) {
        Some(n) => Some((stats::score(freqs, &n), n)),
        None => None
    }
}

fn hex_xor_single(in_str: &str, c: &char) -> Option<String> {
    let bytes_a = match hex::decode(in_str) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };
    let res: Vec<u8> = bytes_a.iter().map(|x| x ^ *c as u8).collect();
    match String::from_utf8(res) {
        Ok(n) => Some(n),
        Err(_e) => None
    }
}

fn hex_xor(in_a: &str, in_b: &str) -> String {
    let bytes_a = match hex::decode(in_a) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };

    let bytes_b = match hex::decode(in_b) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };

    let result: Vec<u8> = bytes_a.iter().zip(bytes_b.iter()).map(|(a, b)| a ^ b).collect();
    
    hex::encode(result)
}