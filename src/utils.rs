use stats;
use std::collections::HashMap;

pub fn xor_vec(in_a: &Vec<u8>, in_b: &Vec<u8>) -> Vec<u8> {
    let result: Vec<u8> = in_a.iter().zip(in_b.iter()).map(|(a, b)| a ^ b).collect();
    result
}

pub fn xor_and_score(in_a: &Vec<u8>, c: &u8, freqs: &HashMap<u8, f32>) -> (f32, Vec<u8>) {
    let res = xor_single(in_a, c);
    (stats::score(freqs, &res), res)
}

pub fn xor_single(in_a: &Vec<u8>, c: &u8) -> Vec<u8> {
    let res: Vec<u8> = in_a.iter().map(|x| x ^ *c).collect();
    res
}

pub fn repeating_key_xor(key: &Vec<u8>, input: &Vec<u8>) -> Vec<u8> {

}