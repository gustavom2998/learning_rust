// Input/Output Library import
use std::io;

// Random number generation library import
use rand::Rng;

// Define main function
fn main(){

    // Identify the game
    println!("Guess the number!");
    
    // Generate a random number and store it in a variable
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    
    // Print the generate number
    println!("The secret number is: {secret_number}");

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