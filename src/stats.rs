use std::collections::HashMap;
use std::f32;

lazy_static! {
    static ref FREQUENCIES: HashMap<u8, f32> = {
        let mut m = HashMap::new();
        m.insert('a' as u8, 0.0651738);
        m.insert('b' as u8, 0.0124248);
        m.insert('c' as u8, 0.0217339);
        m.insert('d' as u8, 0.0349835);
        m.insert('e' as u8, 0.1041442);
        m.insert('f' as u8, 0.0197881);
        m.insert('g' as u8, 0.0158610);
        m.insert('h' as u8, 0.0492888);
        m.insert('i' as u8, 0.0558094);
        m.insert('j' as u8, 0.0009033);
        m.insert('k' as u8, 0.0050529);
        m.insert('l' as u8, 0.0331490);
        m.insert('m' as u8, 0.0202124);
        m.insert('n' as u8, 0.0564513);
        m.insert('o' as u8, 0.0596302);
        m.insert('p' as u8, 0.0137645);
        m.insert('q' as u8, 0.0008606);
        m.insert('r' as u8, 0.0497563);
        m.insert('s' as u8, 0.0515760);
        m.insert('t' as u8, 0.0729357);
        m.insert('u' as u8, 0.0225134);
        m.insert('v' as u8, 0.0082903);
        m.insert('w' as u8, 0.0171272);
        m.insert('x' as u8, 0.0013692);
        m.insert('y' as u8, 0.0145984);
        m.insert('z' as u8, 0.0007836);
        m.insert(' ' as u8, 0.1918182);
        m
    };
}

pub fn init() -> HashMap<u8, f32>{
    FREQUENCIES.clone()
}

pub fn score(freqs: &HashMap<u8, f32>, input: &Vec<u8>, count_ratio: f32) -> f32 {
    let mut count: HashMap<u8, f32> = (0..26).map(|x| (x + 'a' as u8, 0.0)).collect();
    count.insert(' ' as u8, 0.0);
    let mut counted = 0;

    for i in input {
        let c = *i as char;
        match c {
            'a' ... 'z' => {
                let val = count.entry(*i).or_insert(0.0);
                *val += 1.0;
                counted += 1
            },
            'A' ... 'Z' => {
                let val = count.entry(c.to_ascii_lowercase() as u8).or_insert(0.0);
                *val += 1.0;
                counted += 1;
            }
            ' ' => {
                let val = count.entry(*i).or_insert(0.0);
                *val += 1.0;
                counted += 1;
            },
            '!' ... '~' => { continue; },
            '\t' | '\r' | '\n' => {},
            _ => return f32::MAX
        }
    }

    if counted == 0 {
        counted = 1;
    }

    if count_ratio > counted as f32 / input.len() as f32 {
        return f32::MAX
    }

    let mut chi2 = 0.0;
    for (key, val) in count {
        let observed = val;
        let expected = freqs[&key] * counted as f32;
        let delta = observed - expected;
        chi2 += (delta * delta) / expected;
    }

    chi2
}