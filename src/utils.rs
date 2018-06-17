use stats;
use std::collections::HashMap;

pub fn xor_vec(in_a: &Vec<u8>, in_b: &Vec<u8>) -> Vec<u8> {
    if in_a.len() > in_b.len() {
        xor_key(in_a, in_b)
    } else {
        xor_key(in_b, in_a)
    }
}

pub fn xor_and_score(in_a: &Vec<u8>, c: &u8, freqs: &HashMap<u8, f32>, count_ratio: f32) -> (f32, Vec<u8>) {
    let key = vec![*c];
    let res = xor_key(in_a, &key);
    (stats::score(freqs, &res, count_ratio), res)
}

pub fn xor_key(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for c in input.chunks(key.len()) {
        let temp: Vec<u8> = c.iter().zip(key.iter()).map(|(a, b)| a ^ b).collect();
        res.extend(temp);
    }
    res
}