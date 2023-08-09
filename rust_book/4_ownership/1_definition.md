# Ownership: What is it?

- Set of rules that convert how a Rust program manages memory.
- Memory can be managed with different approaches in Programming Languages
    - Garbage collection: Periodically looks for unused memory (Go).
    - Programmer explicitly deallocates memory when it's no longer needed (C).
    - Ownership: Memory is managed via ownership with a set of rules the compile checks for (Rust)

## Stack and Heap

- In rust, if a memory is on the stack or heap affects the languages behaviour (and therefore decisions the developer needs to make)
- Stack and heaps are separate structures that are part of the memory available for the code to use at runtime.
- The stack stores values in the other it gets them (Last in, First Out).
- Data with a known fixed size can be stored on the stack.
- Data with an unknown size at compile time (or it may change) needs to be stored on the heap.
- When data is stored on the heap, a certain amount of space is requested.
- Allocation: The memory allocator finds a space that is big enough, marks it as in use and returns a pointer (The address for the location).
- Pushing onto the stack is not considered allocation.
- Pushing to the stack is faster because there isn't an allocation process.
- Accessing data on the heap is also slower.
- The values passed to a function when it's called get pushed onto the stack.
- When the function is over, those values are popped from the stack.
- Ownership addresses the issues of tracking what data is on the stack/heap, minimizing duplicate data on the heap, cleaning it up, etc.
- It's important to understand the stack and heap to understand how ownership works, and therefore use it well.

## Ownership Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Variable Scope

- The scope is the range within a program for which a variable is valid.
- The value for the variable is valid as long as it remains in scope

```rust
    {                       // s is not valid here, it’s not yet declared
        let s = "hello";    // s is valid from this point forward
        ...
    }                       // this scope is now over, and s is no longer valid
```

## `String` type

- This is a complex data type which is stored on the heap (different from the ones explored on [Data Types](../3_programming_concepts/2_data_types.md)) 
- String literals are hard coded into our program - they're immutable.
- The `String` type can be used when we don't know what the string will contain as we write the program (e.g user input).
- Here's an example of a dynamic string:

```rust
let mut s = String::from("hello"); // Define mutable string

s.push_str(" world!"); // Change string content in run time - add " World" to the end of the string

println!("{}", s); // Print "Hello World!"
```

## Memory and Allocation

- String literals are fast and efficient because the text is hardcoded into the final executable.
- To support mutable, growable text, we need to allocate an amount of memory in the heap.
    - The memory must be required from the allocator at runtime.
    - We need to free this memory once we're done with the string.
- The memory request is done when we call `String::from` - and is normally done in the same way accross programming languages.
- The memory being freed varies from language to language
    - Garbage collectors (GC), Manual free by the user, etc.
    - Rust frees the memory when the variable goes out of scope.
- Rust frees the memory for a variable when it goes out of scope by using the `drop` function.
- This is Resource Acquisition is Initialization (RAII) - and it's done for C++ as well.


## Variables and Data interacting with Move
