# Programming a Guessing game - Comparison

Up to now, we've made a game that generates a random number and takes user input, then ends. In this topic, we will learn how to compare the numbers.

## Comparison

We can add comparison logic to our script with the following code snippet:

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io; 

fn main() {
    // -- snip --

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

```

We can break down the changes made:

- `use std::cmp::Ordering;`: The ordering type is another enum and has the variants `Less`, `Greater` and `Equal`; which are the possible outputs for comparing values to identify an order.
- We then use the `cmp` method which compares two values and can be called on anything that can be compared. We use the method to compare `guess` to secret_number.
- `cmp` returns a variant of the `Ordering` enum.
- The `match` expression is used to decide what to do next based on which variant of Ordering was returned from the call to `cmp`.
  - The `match` expression is made up of `arms`.
  - `arms` are a pattern to match against, and the code that should be run if the value given to match fits the arm's pattern.
  - Rust takes the value given to match and looks through each arm's pattern in turn.
  - The match statement ends its execution after matching any single arm.

## Typing

Since Rust has a strong static type system, the previous code won't actually compile if added to our example. This is because the variable `secret_number` is an integer (by default i32), and the user input is a string.

We can  add the following code snippet to our code before making the comparison to cast the user input to an `i32` data type:

```rust
let guess: u32 = guess.trim()
    .parse()
    .expect("Please type a number!")
```

- We create a variable named `guess` (which is allowed because of shadowing). Shadowing lets us reuse variable names, rather than forcing us to create two unique variable names.
- We then bind the variable to the expression `guess.trim().parse()`
  - `guess` refers to our original string variable
  - `trim()` is a method for strings that eliminate whitespaces and newlines at the beginning and end of the string (which is read by the IO module).
  - `parse()` is a method for strings that converts strings to another type. By defining the variable type beforehand, we tell Rust exactly what type we want. 
    - This method will only work for characters that can be converted to numbers, so it will probably fail.
    - This method returns a `Result` enum which can be used to handle failures.
