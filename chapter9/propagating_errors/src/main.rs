use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    let content = read_contents_from_file("hello.txt").unwrap();

    println!("{}", content);
}

fn read_contents_from_file(file_name: &str) -> Result<String, io::Error> {
    // using match to be explicit


    // let mut file = match File::open(file_name) {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    // let mut content = String::new();

    // match file.read_to_string(&mut content) {
    //     Ok(_) => Ok(content),
    //     Err(error) => Err(error),
    // }


    //using ? to shorten the function


    // let mut file = File::open(file_name)?;
    // let mut content = String::new();
    // file.read_to_string(&mut content)?;
    // Ok(content)


    // chainging to make the function short


    let mut content = String::new();
    File::open(file_name)?.read_to_string(&mut content)?;
    Ok(content)
}