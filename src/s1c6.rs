extern crate hex;
extern crate base64;
extern crate hamming;

use std::collections::HashMap;
use std::f32;
use std::fs::File;
use std::io::{
    BufRead,
    BufReader,
};
use utils;

pub fn solve(path: String, freqs: &HashMap<u8, f32>) {
    let hamming_string1 = "this is a test";
    let hamming_string2 = "wokka wokka!!!";

    assert_eq!(37, hamming::distance(&hamming_string1.as_bytes(), &hamming_string2.as_bytes()));
    let mut line: String = String::new();
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    for l in reader.lines() {
        let temp_line = l.unwrap();
        line.push_str(&temp_line);
    }

    let file_bytes = match hex::decode(line) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };

    let mut smallest_distance = f32::MAX;
    let mut key_candidate = 0;

    for keysize in 2 ..= 40 {
        let mut iter = file_bytes.chunks(keysize);
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let distance = hamming::distance(&first, &second);
        let normalized_distance = distance as f32 / keysize as f32;
        if normalized_distance < smallest_distance {
            smallest_distance = normalized_distance;
            key_candidate = keysize;
        }
    }

    let chunks = file_bytes.chunks(key_candidate);
    let mut buckets: Vec<Vec<u8>> = Vec::new();

    for i in 0 ..= chunks.len() {
        buckets.push(Vec::new());
    }

    for chunk in chunks {
        chunk.iter().enumerate().for_each(|(i, e)| { buckets[i].push(*e)} )
    }

    let mut key: Vec<u8> = Vec::new();

    for block in buckets {
        let mut message_score = f32::MAX;
        let mut message: String;
        let mut line: String;
        let mut xor_char: u8 = 0u8;

        for c in b' ' ..= b'~' {
            let res_tuple = utils::xor_and_score(&block, &c, freqs);
            match String::from_utf8(res_tuple.1) {
                Ok(n) => {
                    if res_tuple.0 < message_score {
                        message_score = res_tuple.0;
                        message = n;
                        xor_char = c;
                    }  
                },
                Err(_e) => continue
            };
        }

        key.push(xor_char);
    }

    let result = utils::xor_key(&file_bytes, &key);
    let decrypted = match String::from_utf8(result) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };

    println!("{}", decrypted);
}