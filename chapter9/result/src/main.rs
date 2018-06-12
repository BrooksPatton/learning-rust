use std::fs::File;

fn main() {
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => panic!("There was an error {:?}", error),
    };
}