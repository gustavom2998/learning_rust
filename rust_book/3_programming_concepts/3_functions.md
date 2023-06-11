# Functions

## Introduction

By now, we've covered `main` functions - the entry points to our programs. 

We've also been using the `fn` keyword to declare new functions. Normally, we use the keyword, followed by a function name, a set of parentheses for parameters and curly brackets for the function body.

**Snake case**: lower case, words separated by underlines. Rust uses snake case as a convention for function and variable names. 

Function declaration order does not matter for rust, so the position for where a function is defined does not influence anything. Functions just need to be defined within a scope to be able to be called.

Below, we declare two functions using the points discussed:

```rust
fn main(){
    println!("Hello world");
    another_function();
}

fn another_function(){
    println!("Another function.")
}
```

## Parameters

Parameters are part of a functions signature. When calling a function with parameters, concrete values (arguments) can be passed as the parameter values.

For example, we could specify a parameter called x which must be a value of type `i32` for our previous snippet:

```rust
fn main(){
    println!("Hello world");
    another_function(1);
}

fn another_function(x: i32){
    println!("The value passed to another function: {x}");
}
```

When specifying parameters, their types must be declared. This is a programming language design decision. This helps the compiler to catch errors and give helpful messages.

## Multiple Parameters

Multiple parameters can be specified by using commas to separate parameters.

```rust
fn main(){
    println!("Hello world");
    another_function(1.5, '$');
}

fn another_function(cost: f64, currency: char){
    println!("The values passed to another function: {}{}", currency, cost);
}
```

## Statements and Expressions

**Statements**: Instructions that perform actions and don't return a value.
- Example: Declaring a value with `let x=1;`
- Example: A function statement `fn func(){}`
- We can identify statements by not being able to atrribute them to a value.
  - Example: `let x=(let y=6);` won't work.


**Expressions**: Evaluate to a resultant value.
- Example: A math operation `5+6` returns the value `11`.
- Example: The `1` inside the statement `let x=1;`.
- Example: Calling a macro.
- Example: A new scope within a curly bracket

```rust
fn main(){
    // The brackets evaluate to 4
    let y = {
        let x = 3;
        x + 1
    };
}
```

## Return values

Functions can return a value to their callers. Values don't need to be named, but they do need to be typed. Types for a function are specified via an arrow (`->`) after the function parameter declaration. The value of a function in Rust is defined by the final expression of the function. The `return` keyword can also be used, but most functions return it implicitly by using the last value.

```rust
fn five() -> i32 {
    5
}

fn main(){
    println!("Five: {}", five())
}
```




