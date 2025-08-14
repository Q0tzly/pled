use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        useage();
        exit(0);
    };

    let content = fs::read_to_string(&args[1]).expect("Error: Can't open the file.");
    println!("{}", content);
}

fn useage() {
    println!("Need just one arg that is file path.")
}
