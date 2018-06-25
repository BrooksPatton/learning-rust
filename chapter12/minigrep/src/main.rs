use std::env;
use std::fs::File;
// use std::io::Read;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let mut file = File::open(filename).expect("Error opening the file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Error reading the file");

    println!("We are searching for {} in {}", query, filename);
    println!("The file contents: {}", file_content);
}
