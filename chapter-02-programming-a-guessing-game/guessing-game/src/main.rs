use std::cmp::Ordering;

// Imports the std io library, used here to get user cmd line input.
use std::io;

use rand::Rng;

fn main() {
    println!("Guess a number!");

    // This is a non mutable value by default in Rust
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // DEBUG code to see the secret number
    //println!("The secret number is: {secret_number}");

    // loop creates an infinite loop
    loop {
        println!("Please input your guess.");

        // Make a mutable variable called guess
        let mut guess = String::new();

        // This line of code returns a Result type.
        // Results type are either Ok values (does intended affect) or Err values (an error occurs).
        io::stdin()
            .read_line(&mut guess) // add the input to the mutable variable guess
            .expect("Failed to read line"); // if we cannot parce, print this.

        // Invalid input handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If Result enum is Ok
            Err(_) => continue, // If Result enum is Err, catch all values _, and restart at the top of the loop
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
