use std::io;
use std::cmp::Ordering;

fn main() {
    let mut secret_number = String::new();
    let max_possible_secret = 100;
    // let min_possible_secret = 1;
    let mut guess = 50;
    let mut last_guess_change_amount = 0;

    println!("Welcome to the reverse guessing game!");
    println!("Please guess a number between 1 and 100");
    
    io::stdin().read_line(&mut secret_number)
        .expect("Getting the number didn't work.");
    
    let secret_number: u32 = secret_number.trim().parse()
        .expect("Converting the secret number to an int didn't work");

    loop {
        println!("I guess {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("I guessed the number!");
                break;
            },
            Ordering::Less => {
                println!("Looks like my guess is a little low");

                if 0 == last_guess_change_amount {
                    last_guess_change_amount = max_possible_secret - guess
                } 
                else {
                    last_guess_change_amount = last_guess_change_amount / 2;
                }

                guess = guess + last_guess_change_amount;
            },
            Ordering::Greater => {
                println!("I'm a little high on my guess");

                if 0 == last_guess_change_amount {
                    last_guess_change_amount = guess / 2;
                } 
                else {
                    last_guess_change_amount = last_guess_change_amount / 2;
                }
                
                guess = guess - last_guess_change_amount;
            },
        }
    }
}
