use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let sample_range = 1..=100; 
    let secret_number: u64 = rand::rng().random_range(sample_range);
    println!("Secret number: {secret_number}");

    println!("Please input your guess.");

    let mut guess: String = String::new();
    let read_response: Result<usize, std::io::Error>;

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
}