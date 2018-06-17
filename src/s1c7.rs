extern crate base64;
extern crate crypto;

use crypto::{aes, blockmodes};
use crypto::buffer::{ RefReadBuffer, RefWriteBuffer, BufferResult };
use std::fs::File;
use std::io::{
    BufRead,
    BufReader,
};

pub fn solve(path: String) {
    let key = "YELLOW SUBMARINE".as_bytes();
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

    let mut dec = vec![0u8; file_bytes.len()];
    {
        let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128,  key, blockmodes::NoPadding);
        let mut read_buf = RefReadBuffer::new(&file_bytes);
        let mut write_buf = RefWriteBuffer::new(&mut dec);

        match decryptor.decrypt(&mut read_buf, &mut write_buf, true) {
            Err(e) => panic!("Decrypt Failed: {:?}", e),
            Ok(res) => match res {
                BufferResult::BufferUnderflow => {},
                BufferResult::BufferOverflow => assert!(false),
            },
        };
    }

    let decrypted = match String::from_utf8(dec) {
        Ok(n) => n,
        Err(e) => panic!(e)
    };
    println!("{}", decrypted);
}