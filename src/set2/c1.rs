use utils;

pub fn solve() {
    let input = "YELLOW SUBMARINE".as_bytes();
    let block_size = 20;

    let res = utils::pad(&input.to_vec(), block_size);
    
    println!("{:?}", res);
    assert_eq!("YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes().to_vec(), res);
}