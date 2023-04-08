// Input/Output Library import
use std::io;

// Define main function
fn main(){

    // Identify the game
    println!("Guess the number");

    // Ask for user input
    println!("Please input your guess.");

    // Declare variable for user input
    let mut guess = String::new();

    // Store user input in variable using stdin command
    io::stdin()
        // Read and store line within the guess variable
        .read_line(&mut guess)
        // If the readline result is error - print error message
        .expect("Failed to read line");

    // Print the users guess
    println!("You guessed: {guess}");
}