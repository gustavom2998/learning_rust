# Control Flow

- Based of the ability to run code based on meeting or not meeting a condition.
- Normally composed of `if` expressions and loops.

## `if` expressions

- Allows us to branch code based on conditions.
- If the condition is met, the chunk of code for the `if` block is executed.
- If the condition isn't met, the chunk of code for the `else` block is executed.
- The following example checks for if a number is smaller than 5 and prints a different message according to the result.
- Else blocks are optional, and in case the `if` condition isn't true and there is no else block, the if block is simply "skipped" (nothing is done).
- We can eexperiment with the following chunk, setting the number to `7` will print the second message.

```rust
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

- Conditions for `if` statements must always be a bool typed value.
- Using values that aren't bools generates an error.
