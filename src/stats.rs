use std::collections::HashMap;
use std::f32;

lazy_static! {
    static ref FREQUENCIES: HashMap<char, f32> = {
        let mut m = HashMap::new();
        m.insert('a', 0.0815);
        m.insert('b', 0.0144);
        m.insert('c', 0.0276);
        m.insert('d', 0.0379);
        m.insert('e', 0.1311);
        m.insert('f', 0.0292);
        m.insert('g', 0.0199);
        m.insert('h', 0.0526);
        m.insert('i', 0.0635);
        m.insert('j', 0.0013);
        m.insert('k', 0.0042);
        m.insert('l', 0.0339);
        m.insert('m', 0.0254);
        m.insert('n', 0.0710);
        m.insert('o', 0.08);
        m.insert('p', 0.0198);
        m.insert('q', 0.0012);
        m.insert('r', 0.0683);
        m.insert('s', 0.0610);
        m.insert('t', 0.1047);
        m.insert('u', 0.0246);
        m.insert('v', 0.0092);
        m.insert('w', 0.0154);
        m.insert('x', 0.0017);
        m.insert('y', 0.0198);
        m.insert('z', 0.0008);
        m
    };
}

pub fn init() -> HashMap<char, f32>{
    FREQUENCIES.clone()
}

pub fn score(freqs: &HashMap<char, f32>, input: &str) -> f32 {
    let mut count: HashMap<char, f32> = HashMap::new();
    let mut ignored = 0;
    let mut spaces = 0;
    for c in input.chars() {
        let char_val = c as u8;
        if !c.is_ascii_alphabetic() {
            if char_val >= 32 && char_val <= 126 || char_val == 9 || char_val == 10 || char_val == 13
            {
                ignored += 1;
                continue;
            }
            return f32::MAX;
        }
        
        let val = count.entry(c.to_ascii_lowercase()).or_insert(0.0);
        *val += 1.0;
    }

    let mut chi2 = 0.0;
    let len = input.len() - ignored;
    let ratio = ignored as f32 / input.len() as f32;
    if ratio > 0.25 {
        return f32::MAX;
    }

    for (key, val) in count {
        let observed = val;
        let expected = len as f32 * freqs[&key];
        let delta = observed - expected;
        chi2 += delta * delta / expected;
    }

    chi2
}