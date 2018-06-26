extern crate base64;
extern crate crypto;
extern crate hamming;
extern crate hex;

use crypto::{aes, blockmodes};
use crypto::buffer::{ RefReadBuffer, RefWriteBuffer, BufferResult };
use std::collections::HashMap;
use std::fs::File;
use std::io::{
    BufRead,
    BufReader,
};

pub fn solve(path: String) {
    let hamming_string2 = "wokka wokka!!!";

    assert_eq!(0, hamming::distance(&hamming_string2.as_bytes(), &hamming_string2.as_bytes()));

    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    for l in reader.lines() {
        let temp_line = l.unwrap();
        let line_bytes = match hex::decode(&temp_line) {
            Ok(n) => n,
            Err(e) => panic!(e)
        };

        let chunks = line_bytes.chunks(16);
        let mut map: HashMap<Vec<u8>, u32> = HashMap::new();

        for c in chunks {
            let val = map.entry(c.to_vec()).or_insert(0);
            *val += 1;
        }

        if map.iter().any(|(_, y)| *y > 1) {
            println!("{}", temp_line);
        }
    }
}