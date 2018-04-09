extern crate rand;

use std::io;
use rand::{Rng, thread_rng};

fn main() {
    println!("Guess the number!");
    println!("Please enter your guess");

    let secret_number = thread_rng().gen_range(1, 101);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}, the secret number is: {}", guess, secret_number) ;
}