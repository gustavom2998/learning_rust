// Input/Output Library import
use std::io;

// Random number generation library import
use rand::Rng;

// Comparison library import - object ordering enum variants
use std::cmp::Ordering;

// Define main function
fn main(){

    // Identify the game
    println!("Guess the number!");
    
    // Generate a random number and store it in a variable
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    
    // Print the generated number
    // println!("The secret number is: {secret_number}");

    // Loop user inputs until he gets the correct answer
    loop {

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
        
        
        // Match the parse Result - Skip iteration if any error happens
        let guess: u32 = match guess
            // Remove spaces from the user guess
            .trim()
            // Parse the input from the user to the variable data type
                .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        // Print the users guess
        println!("You guessed: {guess}");

        // Use string comparison and compare the result with the possible states to decide what to do
        match guess.cmp(&secret_number){
            // If guess < secret_number - too small
            Ordering::Less => println!("Too small!"),
            // If guess > secret_number - too big
            Ordering::Greater => println!("Too big!"),
            // If guess = secret_number - user wins
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}