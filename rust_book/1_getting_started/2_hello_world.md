# Hello, World

Create a new file and call it `hello_world.rs`. This is where we'll put our Rust code. Then, type the following code into the file:

```rust
fn main() {
    println!("Hello, world!");
}
```

Save the file. We can compile in the terminal with the following command:

```bash
rustc rust_book/1_getting_started/hello_world.rs
```

We can the run the compiled program with the following command:

```bash
.\hello_world.exe
```

We can break down what we just did in the previous block of code.

The function statement `fn main(){}`: 
- Defines a function named main. 
- This is the entry point of our program. 
- The function has no parameters and it doesn't return anything.  
- If there were parameters, they would be provided within the parenthesis.
- The function body is wrapped in curly braces. 
- All functions must be wrapped in curly braces in rust.
- It's good practice to place the opening curly brace on the same line as the function declaration.

The function body `println!("Hello, world!");`:
- Rust identation uses 4 spaces and not tabs.
- The `println!` macro prints text to the console. We can identify it as a macro (and not a function) because it used an exclamation mark.
- We pass the string `"Hello, world!"` to the macro as an argument.
- We end the line with a semicolon. Most lines of rust code end with a semicolon.


## Compiling vs Running

Before running a rust program, we must compile it using the rust compiler by entering the rustc command and passing it the name of the source file:
    
```bash
rustc hello_world.rs
```

Files with source code can be identified by the the `.rs` extension. In linux - compiled files don't have an extension. On Windows, compiled files have the `.exe` extension.

As an overview, differently to dynamic languages like Python, rust is an ahead of time compiled language. We compile the code into an executable and some one can run the code even if they don't have rust installed.

Compiling with rustc is okay for smaller languages, but we can se Cargo for larger projects, which we'll see in the next topic.