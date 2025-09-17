use std::{io, ops::RangeInclusive, cmp::Ordering};
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let sample_range: RangeInclusive<u8> = 1..=100;
    let secret_number: u8 = rand::rng().random_range(sample_range);
    println!("Secret number: {secret_number}");

    println!("Please input your guess.");

    let mut guess: String = String::new();
    let mut read_response: Result<usize, std::io::Error>;

    loop {

        read_response = io::stdin()
            .read_line(&mut guess);

        let n_outer: usize = match read_response {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {e}");
                // returns a never type...
                return
            }
        };

        println!("You wrote {n_outer} bytes");
        println!("You guessed: {guess}");

        println!("Convert guess to a number");

        let guess_parse_result: Result<u8, std::num::ParseIntError> = guess.trim().parse::<u8>();

        let guess_number: u8 = match guess_parse_result {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {e}");
                println!("Please type a number!");
                guess.clear();
                // returns a never type...
                continue;
            }
        };
    
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
        }

        guess.clear();
    }

}