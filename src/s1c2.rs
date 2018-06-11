extern crate hex;

use utils;

pub fn solve() {
    let bytes_a = match hex::decode("1c0111001f010100061a024b53535009181c") {
        Ok(n) => n,
        Err(e) => panic!(e)
    };

    let bytes_b = match hex::decode("686974207468652062756c6c277320657965") {
        Ok(n) => n,
        Err(e) => panic!(e)
    };

    let res = utils::xor_vec(&bytes_a, &bytes_b);
    let res_str = hex::encode(res);
    println!("{}", res_str);
    assert_eq!("746865206b696420646f6e277420706c6179", res_str);
}