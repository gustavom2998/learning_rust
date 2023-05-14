# Comments


## Introduction

- Comments are useful for helping readers of the code. 
- The compiler ignores them.
- Comments in rust are defined by the two slashes `//` followed by the comment content.
  
## Single line comment

```rust
fn main(){
    // Our first comment
    println!("Hello world")
}
```

## Multi line comment

- Rust doesn't have a set of characters for commenting out a block of lines.
- We must use `//` in each line for the block.
- An IDE can help with commenting out blocks more easily.

```rust
fn main(){
    // Our first comment
    // This function prints hello world
    // Author: Gustavo
    println!("Hello world");
}
```

## Inline comment

- We can also put a comment inside a code line, as follows:

```rust
fn main(){
    println!("Hello world"); // This function prints hello world
}
```

## Useful documentation comments

- Explored further in chapter 14.
- Covers building out Crates, documenting and publishing them.
-  Covers documenting advanced topics we havn't covered yet.