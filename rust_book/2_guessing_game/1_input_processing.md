# Programming a Guessing game - Input Processing

In this topic we're going to build a guessing game. The program should generate a random integer between 1 and 100. It will prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will congratulate the player and exit.

Let's make a new project with Cargo called `guessing_game`:

```powershell
cargo new guessing_game
```

We can test the new project with, which will run the hello world script:

```powershell
cargo run
```

## Processing a Guess

We can start the game  asking for use input and checking the user input. We can write the following main file:

```rust
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

    println!("You guessed: {guess}");
}
```

We can explore the code by parts of bits that we've already explored before:
- `use std::io;` - Library import for input/output library
  - `std` - Identifies the standard library 
  - *prelude* = Items within the standard library which are brought into the scope of every program
  - One of the features of the `io` library is the ability to accept user input
- `fn main(){}` - Entry point to the program.
  - `fn` is used to declare a new function
  - `()` indicicates there are no parameters
  - `{}` indicates the function body
- `println!` - Macro that prints a string to the screen
  - The first string identifies the game
  - The second string prompts for user input

## Variables

The `let mut guess = String::new()` line is a new piece of code which we havn't explored yet.

Lets begin by declaring a variable called apples, which is an integer and binds it to the number 5:

```rust
let apples = 5;
```

One observation is that variables in Rust are immutable by default. Once we define a value, the value can't change. We can make the variable mutable by adding the `mut` keyword before the variable name:

```rust
// Imutable
let apples = 5;

// Mutable
let mut bananas = 5;
```

Going back to our original snippet of code: `let mut guess = String::new()` - The `=` operator indicates we want to bind a value to a variable. On the right of the equal is the value that guess is bound to - which is the result of calling `String::new()`.

 The `String` keyword is a type within the standard library that is a growable, UTF-8 encoded bit of text. The `::` syntax is an association function thats implemented for a specific type, in this case `String`. The `new` function creates a new, empty String. It's a common naming convention for creating new obejcts of different data types. 

 ## User Input

We included the input/output functionality by importing the `io` module from the standard library. We then call the `stdin` function from this module which allows us to handle user input:

```rust
io::stdin()
    .read_line(&mut guess)
```

If we hand't imported the library at the beginning of the library, we could still use this function by using it as `std::io::stdin()`. This function returns an instance of `std::io::Stdin` - which is a type that represents a handle to the standard input for our terminal.

The `.read_line(&mut guess)` calls the read_line method on the standard input handle toget input from the user. We're alsopassing `&mut guess` as an argument to tell it to store a string in the guess variable. The argument needs to be mutable so the method can change the strings content. 

The `&` indicates that this argument is a reference, which gives us a way to let multiple parts of our code access one piece of data without needing to copy that data int memory multiple times.

## Error handling

The `.expect("Failed to read line");` is basically on the same line as the previous command - we just broke the line visually with the new line - without using a `;` to indicate an end of line.

`read_line` puts whatever the user enters intothe string we pass to it, but it also returns a `Result` type value.`Result` is an enumeration (enum) - which can be one of multiple possible states. Each state is called a `variant`. 

The variants encode error-handling information in this case. They can either be `Ok` or `Err`. The `Err` variant contains information about why the operation failed.

The `Result` type has a set of mthods defined within it. It has an `expect` method which can be called to cause the program to crash and display a message if the instance of `Result` is an `Err` value.

We could instead not use `expect` - and the program would compile, but we would get a compiler warning about an Unused result. We don't actully handle the error with expect, we just make sure to crash the program if an error happens.

## String placeholders

We still have the `println!("You guessed: {guess}");` left. The curly brace `{}` within the string is used to hold a value in place within the string. We can put the variable name within `{}` to print the value of the variable (instead of printing the variable name). 


We can also use empty braces to provide the expression after the string, as follows:

```rust
let x=5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2)
```

This prints the folllowing output: `x = 5 and y + 2 = 12`. Expressions can't be placed within the string, so we have to put it outside.

## Testing

Lets copy and paste the snippet from the first part into our `main.rs` file and run the code. We should be prompeted to input a number and the program should end after saying what number we entered.

```powershell
cargo run
```

Heres the output for the interaction:

```powershell
Finished dev [unoptimized + debuginfo] target(s) in 0.90s
     Running `target\debug\guessing_game.exe`
Guess the number
Please input your guess.
5
You guessed: 5
```