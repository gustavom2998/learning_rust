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

## Repetition expressions

- Useful for executing the same block of code more than once.
- `loop`, `while`, `for`

### `loop`

- Execute a block of code forever until explicitly stopped.
- Stop by sending `SIGINT` (press `ctrl+c` in the terminal running the process) 
- Stop by using `break` keyword.
- Skip any remaining code by using `continue` keyword.

```rust
fn main(){
    let num = 8;
    let mut counter = 0;

    println!("Count to {num} skipping odd numbers");
    loop{
        counter += 1;
        if (counter % 2 == 1) && (counter <= num) {
            continue;
        }
        else if counter <= num {
            println!("Current count: {counter}/{num}");
        }
        else {
            println!("Finished count");
            break;
        }
    }
}
```

- We can also return values from loops.
- This is useful for retrying operations that may fail until they succeed.
- We can add the returned value to the end of the break expression.

```rust
fn main (){
    let num = 8;
    let mut counter = 0;
    let mut sum = 0;

    let sum_res = loop{
        counter += 1;
        sum += counter if (counter % 2 == 0);

        if counter > num {
            break sum;
        }
    }
    println!("Sumation to {num} skipping odd numbers: {sum_res}");
}
```

- Loop labels can be used to help with nested loops.
- These are used in combination with `break` and `continue` keywords
- Great to specify what loop to apply the operations to.

```rust
fn main (){
    let num = 4;
    let mut counter = 0;
    let mut sum = 0;

    'sum_facts : loop { 
        counter += 1;
        let mut fact_val = 1;
        let mut fact_counter = num;

        'fact : loop {
            fact_val *= fact_counter;
            fact_counter -= 1;

            if fact_counter <= 1 {
                break 'fact;
            }
        }

        sum += if counter % 2 == 0 {fact_val} else {0}; 

        if counter > num {
            break 'sum_facts;
        }
    }
    println!("Sumation of factorial(n={num}) skipping odd n's: {sum}");
}
```

### Satisfying conditions - `while`

- While loops are good for running a chunk of code until a condition stops being true
- This avoids having to have an `if` + `break` within a `loop`
- We can use pass a condition to after the `while` keyword and declare the repeated code block.

```rust
fn main(){
    let mut countdown = 5;

    while countdown > 0 {
        println!("{countdown}");
        countdown -= 1;
    }
    println!("Liftoff!");
}
```

### Processing collections - `for`

- The `while` loop can be used to loop over collections
- However, we need to initialize and control an `index` counter
- It's easier to use `for`
- This is a safer operation that avoids common problems
    - Accessing unavailable index
    - Forgetting to increment counters

```rust
fn main(){
    let arr = [1,2,3,4];

    for item in arr {
        println!("{item}");
    }
}
```

- A can be used to generate a sequence of numbers
- It can be identified by  2 values, a `start` and `end` for the range, separate by a `..`
- This also helps with processing sequences (even if we don't naturally iterative values)
- The step can be given by using the method `step_by(n)`to provide a custom count step.
- The `rev` method can be used to get a reversed range.
- The countdown function will look a lot cleaner using a for with range.

```rust
fn main(){
    let countdown = 5;
    for item in (1..countdown + 1).rev() {
        println!("{item}");
    }
    println!("Liftoff!");
}
```
