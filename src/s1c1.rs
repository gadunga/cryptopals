extern crate hex;
extern crate base64;

pub fn solve() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = match hex::decode(input) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };
    
    println!("{}", base64::encode(&bytes));
    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", base64::encode(&bytes));
}