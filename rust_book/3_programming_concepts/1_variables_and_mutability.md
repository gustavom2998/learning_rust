# Variables and Mutability

By default, rust variables are immutable. Variables can be made into mutable variables - and we can explore reasons to favor immutability and why sometimes we require mutability.

We will create a new project called variables: `cargo new variables`

We can then add the following piece of code to our project: 

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

The script has an error - `x` is being modified even though it is immutable. If we run this script with `cargo run` - we will get a compiler error.

Mutability can be very useful - and we can make our code mutable by adding the `mut` keyword to our code. This is helpful since it indicates that the writer of the code intended to reuse the variable and change its value.

Adding this to the script now lets us execute it:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

## Constants

Like immutable variables, constants are values that are bound to a name and are not allowed to change. However, there are some differences between constants and variables:


- Constants cant be used with the `mut` keyword - they're always immutable
- Constants are declared with the `const` keyword instead of the `let` keyword
- The type of the constant must always be annotated
- Constants can be declared in any scope, including global, which makes them useful for values that need to be reused throughout the code and don't change much.
- Constants may be set to only a constant expression, not the result of a value that could only be computed at run time.

Below, we show the example of a constant:

```rust
const THREE_HOURS_IN_SECONDS = u32 = 60 * 60 * 3;
```

- The constants name is `THREE_HOURS_IN_SECONDS`
- Its value is set to the result of multiple 3 integers
- The Rust naming convention for constants is all uppercase with underscores as separators.
  

## Shadowing

A new variable can be declared with the same name as a previous variable. It's common to say that the first variable is *shadowed* by the second, which means that the second variable is what the compiler will see when you use the name of the variable. A variable can be shadowed by repeating its name and using the `let` keyword to define the variable. The value will be defined until the variable is refined or when the scope ends. For example:

```rust
fn main(){
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2
        println("The value of X")
    }
}
```

Shadowing is different to marking a variable as mutable. If we accidentally try to overwrite the value without using let, we'll get an error. We can use shadowing to process and transform data without having to create new variables for each step. For example, we may want to declare a string of spaces, and then use the same variable to calculate the number of spaces.

```rust
fn main(){
    // String: A string containing a sequence of spaces
    let spaces = "     "; 

    // Shadowing to redefine what spaces is
    // Integer: Number of space characters within the spaces string previously.
    let spaces = spaces.len();
    
    println!("{}",spaces);
}

```

We could try rewriting this code using mut, but it would fail. This is because mut allows the value of the variable to change, but not its type.

```rust
fn main(){
    // String: A string containing a sequence of spaces
    let mut spaces = "     "; 

    // Shadowing to redefine what spaces is
    // error: Number of space characters within the spaces string previously.
    spaces = spaces.len();
    
    println!("{}",spaces);
}

```
