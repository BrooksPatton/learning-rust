extern crate minigrep;
use std::env;
use minigrep::grep;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    let case_sensitive_search = env::var("CASE_SENSITIVE").is_err();
    let config = grep::Config::new(args, case_sensitive_search)
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });

    match grep::run(config) {
        Ok(text) => println!("{}", text),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
}
