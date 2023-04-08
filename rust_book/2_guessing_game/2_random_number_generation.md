# Programming a Guessing game - Random Number Generation

We've already worked on getting the user input and storing it for our guessing game. The next step it to increment our code so that we can generate a secret number.

## Adding Crates

Currently rust doesn't include random number generation in the standard library, but the `rand` crate can be used.

We can add this crate by modifying the `Cargo.toml` file or by adding it via CLI:

```rust
cargo add rand@0.8.5
```

The previous command adds the depedency to the `Cargo.toml` file and pre-downloads the library and its dependencies from [Crates.io](https://crates.io). 

We can then and runs the `cargo build` command, which adds the crate to our project and updates our `Cargo.lock` file. Cargo figures out all of the versions of our dependencies that meet the criteria especified in the Toml file and stores these versions in the lock file.

## Updating Crates

The `update` commnd can be used to ignore the `Cargo.lock` file and 
figure out  and get the latest versions that meet the `Cargo.toml` file.

```powershell
cargo update

```

## Random Number Generation

We can start using the `rand` library to generate random numbers. We can write our `main.rs` script to contain the following contents:

```rust
use std::io;
use rand::Rng;

fn main(){
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

Now to explore what the updated script does:
- `use rand::Rng` - The `Rng` trait defines methods that random number generators implement, and this trati must be in scope for us to use these methods.
- `rand::thread_rng` - This is a function that gives us a particular random number generator - local to current thread of execution and is seeded by the operating system.
- `gen_range()` - Method called on the random number generator selected. Defined by the Rng trait we imported. It takes a range expression as an argument and generates a random number in the range.
- Range expressions are defined by the following syntax: `start..=end` - it's inclusive on lower and upper bounds.