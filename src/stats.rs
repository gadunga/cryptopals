use std::collections::HashMap;
use std::f32;

lazy_static! {
    static ref FREQUENCIES: HashMap<u8, f32> = {
        let mut m = HashMap::new();
        m.insert('a' as u8, 0.0815);
        m.insert('b' as u8, 0.0144);
        m.insert('c' as u8, 0.0276);
        m.insert('d' as u8, 0.0379);
        m.insert('e' as u8, 0.1311);
        m.insert('f' as u8, 0.0292);
        m.insert('g' as u8, 0.0199);
        m.insert('h' as u8, 0.0526);
        m.insert('i' as u8, 0.0635);
        m.insert('j' as u8, 0.0013);
        m.insert('k' as u8, 0.0042);
        m.insert('l' as u8, 0.0339);
        m.insert('m' as u8, 0.0254);
        m.insert('n' as u8, 0.0710);
        m.insert('o' as u8, 0.08);
        m.insert('p' as u8, 0.0198);
        m.insert('q' as u8, 0.0012);
        m.insert('r' as u8, 0.0683);
        m.insert('s' as u8, 0.0610);
        m.insert('t' as u8, 0.1047);
        m.insert('u' as u8, 0.0246);
        m.insert('v' as u8, 0.0092);
        m.insert('w' as u8, 0.0154);
        m.insert('x' as u8, 0.0017);
        m.insert('y' as u8, 0.0198);
        m.insert('z' as u8, 0.0008);
        m
    };
}

pub fn init() -> HashMap<u8, f32>{
    FREQUENCIES.clone()
}

pub fn score(freqs: &HashMap<u8, f32>, input: &Vec<u8>) -> f32 {
    let mut count: HashMap<u8, f32> = HashMap::new();
    let mut ignored = 0;

    for c in input {
        let char_val = *c as char;
        if !char_val.is_ascii_alphabetic() {
            if *c >= 32u8 && *c <= 126u8 || *c == 9u8 || *c == 10u8 || *c == 13u8
            {
                ignored += 1;
                continue;
            }
            return f32::MAX;
        }
        
        let val = count.entry(char_val.to_ascii_lowercase() as u8).or_insert(0.0);
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