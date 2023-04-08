# Programming a Guessing game - Repetition

We've built a game capable of generating a random number, receiving a user input and comparing the two. The problem is our program ends after performing the comparison. Our original objective was to allow the user to retry multiple times until he got it right. We'll be using this by adding repetion structures to our game.

## Looping

The `loop` keyword can be used to create an infinite loop. We'll add a loop to give users more changes at guessing the number:

```rust
loop {
    // -- Input guess code

    match guess.cmp(&secret_number){
        // -- Ordering Enum branches 
    }
}
```

We should move both the user input and the comparison into the loop to get the desired behaviour. If we run the program after simply adding the code, we will run into the problem of having no exit clause.

## Exit Clause

We can break our app by typing in anything that isn't a number, but Ideally we want the game to properly exit when the user gets the answer correct. 

We can do this by adding a block of code to the Equals comparison operation. We can then add the `break` keyword to break out of the loop. After adding this, our program will also end since there is no more code to process within the main function. Here a snippet with the updated code:

```rust
loop {
    // -- Input guess code

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
}
```

## Error Handling

To finish up our game, we can make the game ignore non-numerical inputs from the user. 

We can do this by adding a match to the number conversion we generate this error:

```rust
let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```

- The `match` expression receives the `Result` from the `parse` method
- If the result is an `Ok` - it returns an `Ok` that contains the number.
- If the result is an `Err` - it returns an `Err` - and in this case we match any error by using the `(_)` comparison - which is a catch all comparison.
- This will cause the program to proceed if a number is correctly read, but will ask for a new number if it fails for any reason.
  
Because of this, the user can now only leave the game by winning the game (or by killing the process).

## Finishing Up

The last step so that we can finish up our project is to remove the print for the secret number. We can do this by commenting it out.