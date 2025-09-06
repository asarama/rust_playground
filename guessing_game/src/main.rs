use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let bytes_read = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You wrote {bytes_read} bytes");
    println!("You guessed: {guess}");
}