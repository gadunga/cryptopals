extern crate hex;
extern crate base64;

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = match hex::decode(input) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };
    
    println!("{}", base64::encode(&bytes));
    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", base64::encode(&bytes));

    let mut res_2 = hex_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965");
    println!("{}", res_2);
    assert_eq!("746865206b696420646f6e277420706c6179", res_2);
    
    for c in b'a' ..= b'z' {
        let temp = hex_xor_single("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736", &(c as char));
        let score = score(&temp);

        println!("{} - {} - {}", c as char, score, temp);
    }
}

fn hex_xor_single(in_str: &str, c: &char) -> String {
    let mut res = Vec::new();

    for x in in_str.chars() {
        res.push(hex_xor(&x.to_string(), &c.to_string()));
    }

    res.join("")
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

    let mut result: Vec<u8> = Vec::new();

    for (x, y) in bytes_a.iter().zip(bytes_b.iter()) {
        result.push(x ^ y);
    }
    
    hex::encode(result)
}

fn score(a: &String) -> f32 {
    let expected = vec![ 0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015,  // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749,  // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758,  // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074];                     // V-Z

    let mut count = vec![0; 26];
    let mut ignored = 0;
    for c in a.chars() {
        if 'A' <= c && c <= 'Z'{
            count[(c as u8 - 'A' as u8) as usize] += 1;
        } else if 'a' <= c && c <= 'z' {
            count[(c as u8 - 'a' as u8) as usize] += 1;
        } else {
            ignored += 1;
        }
    }

    let mut chi2 = 0.0;
    let len = a.len() - ignored;

    for (x, y) in count.iter().zip(expected.iter()) {
        let observed = x;
        let expected = len as f32 * y;
        let delta = *observed as f32 - expected;
        chi2 += delta * delta / expected;
    }

    chi2
}