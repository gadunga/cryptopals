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

    let file_bytes = match base64::decode(&line) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };

    let key_candidates = get_key_candidates(&file_bytes);
    for key in key_candidates {
        test_key_size(key, &file_bytes, freqs)
    }
}

fn get_key_candidates(bytes: &Vec<u8>) -> Vec<usize> {
    let mut pairs: Vec<(f32, usize)> = Vec::new();

    for keysize in 2 ..= 40 {
        let mut iter = bytes.chunks(keysize);
        let distance1 = hamming::distance(&iter.next().unwrap(), &iter.next().unwrap());
        let distance2 = hamming::distance(&iter.next().unwrap(), &iter.next().unwrap());
        let average: f32 = (distance1 as f32 + distance2 as f32) / 2.0;
        let normalized_distance = average / keysize as f32;
        pairs.push((normalized_distance, keysize));
    }

    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    pairs.iter().map(|a| a.1).collect()
}

fn test_key_size(key_size: usize, bytes: &Vec<u8>, freqs: &HashMap<u8, f32>) {
    let chunks = bytes.chunks(key_size);
    let mut buckets: Vec<Vec<u8>> = Vec::new();

    for _i in 0 .. key_size {
        buckets.push(Vec::new());
    }
    chunks.for_each(
        |e| e.iter().enumerate().for_each(
            |(i, e)| buckets[i].push(*e)));

    let mut key: Vec<u8> = Vec::new();

    for block in buckets {
        let mut message_score = f32::MAX;
        let mut xor_char: u8 = 0u8;

        for c in b' ' ..= b'~' {
            let res_tuple = utils::xor_and_score(&block, &c, freqs, 0.8);
            if res_tuple.0 < message_score {
                message_score = res_tuple.0;
                xor_char = c;
            };
        }

        if message_score != f32::MAX {
            key.push(xor_char);
        }
    }

    if key.len() == 0 {
        return;
    }

    let result = utils::xor_key(&bytes, &key);
    let decrypted = match String::from_utf8(result) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };
    println!("key: {:?}", key);
    println!("{}", decrypted);
}