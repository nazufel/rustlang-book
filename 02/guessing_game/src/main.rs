use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("random number: {}", secret_number);

    loop {
        println!("Please input a guess.");

        // create a new mut variable of type string to store user input
        let mut guess = String::new();
    
        // use the standard input to listen for user keystrokes
        io::stdin()
            // read_line can reate the variable, but does not take ownership of it
            .read_line(&mut guess)
            // read_line returns a result type. expect will catch it and if OK, 
            // continue, but if there's an error, it will print the defined error.
            .expect("Failed to read line");
        
        // need to convert the guess from a string to an unsigned 32-bit int
        // the parse method returns a Result type, so use exepct() to catch the error.
        let guess: u32 = match guess.trim().parse() {
            // first variant is Ok, if Ok, return the value
            Ok(num) => num,
            // if the variant is an error, then continue the loop at the top
            // and ask for a guess again.
            Err(_) => continue,
        
        };
        // print the user's guess they input
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too large!".red()),
            // if the player guesses the correct number, then they break out of the
            // of the loop and end the game
            Ordering::Equal => {
                println!("{}","You guessed the number! You win!".green());
                break;
            }
        }
    }
}
