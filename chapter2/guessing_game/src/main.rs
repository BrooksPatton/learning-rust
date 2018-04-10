extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1, 101);
    
    loop {
        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }
    }
}