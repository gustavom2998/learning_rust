# Control Flow

- Based on the ability to run code based on meeting or not meeting a condition.
- Normally composed of `if` expressions and loops.

## `if` expressions

- Allows us to branch code based on conditions.
- If the condition is met, the chunk of code for the `if` block is executed.
- If the condition isn't met, the chunk of code for the `else` block is executed.
- The following example checks for if a number is smaller than 5 and prints a different message according to the result.
- Else blocks are optional, and in case the `if` condition isn't true and there is no else block, the if block is simply "skipped" (nothing is done).
- We can experiment with the following chunk, setting the number to `7` will print the second message.

```rustPg
fn main(){
    let number = 3;
    if number < 5 {
        println!("Number is smaller than 5");
    }
    else {
        println!("Number is not smaller than 5");
    }
}
```

- Conditions for `if` statements must always be a bool-typed value.
- Using values that aren't bools generates an error.

### Multiple conditions

- We can begin with an if block
- Chain as many else ifs we want (optional)
- End it with an else statement (optional)
- Can be kind of cluttering (replace with `match`)

```rust
fn main(){
    let number = 13;

    if number % 2 == 0 {
        println!("Number is divisible by 2");
    }
    else if number % 3 == 0 {
        println!("Number is divisible by 3");
    }
    else if number % 4 == 0 {
        println!("Number is divisible by 4");
    }
    else {
        println!("Number is not divisible by 2,3 or 4.");
    }
}
```

### Ternaries

- Ifs can be used to define values in a single line.
- When using them like this, we need to be careful to return the same type.

```rust
fn main(){
    let condition = true;
    let number = if condition {1} else {0};
    println!("Number: {number}")
}
```

