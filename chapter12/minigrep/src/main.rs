extern crate minigrep;
use std::env;
use minigrep::grep;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    let config = grep::Config::new(args, false)
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });
    match grep::run(config) {
        Ok(text) => println!("{}", text),
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
}
