use std::fs;

fn main() {
    let mut buf = [0u8; 32];

    getrandom::getrandom(&mut buf).unwrap();

    let total: u64 = buf.into_iter().map(|b| b as u64).sum();

    fs::write("size.txt", total.to_string()).unwrap();
}
