extern crate minigrep;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use minigrep::grep;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    let config = grep::Config::new(args)?;

    let mut file = File::open(&config.filename).expect("Error opening the file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Error reading the file");

    println!("We are searching for {} in {}", config.query, config.filename);
    println!("The file contents: {}", file_content);

    Ok(())
}
