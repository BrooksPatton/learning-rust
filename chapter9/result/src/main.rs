use std::fs::File;
// use std::io::ErrorKind;
// use std::io::BufReader;
// use std::io::Write;
use std::io::Read;
use std::io::Error;

fn main() {
    // let f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     // Err(error) => panic!("There was an error {:?}", error.kind()),
    //     Err(error) => {
    //         let reason = error.kind();
    //         if reason == ErrorKind::NotFound {
    //             match File::create("hello.txt") {
    //                 Ok(mut file) => {
    //                     match file.write_all(b"I have been created!") {
    //                         Ok(_) => file,
    //                         Err(error) => panic!("Error writing to the file, {:?}", error),
    //                     }
    //                 },
    //                 Err(error) => panic!("Error creating the file: {:?}", error),
    //             }
    //         } else {
    //             panic!("There was an error: {:?}", error);
    //         }
    //     }
    // };


    // let f = File::open("hello.txt")
    //     .expect("Error opening the file");

    // let mut hello_buffer = BufReader::new(f);
    // let mut hello_contents = String::new();
    
    // match hello_buffer.read_to_string(&mut hello_contents){
    //     Ok(_) => println!("The file contains: {}", hello_contents),
    //     Err(error) => panic!("error reading the file: {:?}", error), //errors out because the file type from write all is different than open
    // };

    let file_content = read_file().expect("error getting the file");
    println!("The file contains: {}", file_content);
}

fn read_file() -> Result<String, Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut content = String::new();
    match f.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(error) => Err(error),
    }
}