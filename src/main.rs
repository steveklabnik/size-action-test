use std::{env, fs, str};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf).unwrap();

    let total: u64 = buf.into_iter().map(|b| b as u64).sum();

    if args.len() == 1 {
        // if we have no argument, generate the file
        fs::write("size.txt", total.to_string()).unwrap();
    } else {
        // if we do have an argument, read the file and compare
        let file = &args[1];

        let new_total = fs::read(file).unwrap();
        let new_total = str::from_utf8(&new_total).unwrap();
        let new_total: u64 = new_total.parse().unwrap();

        let difference = i128::from(total) - i128::from(new_total);

        println!("{total} - {new_total} = {difference}");
    }
}
