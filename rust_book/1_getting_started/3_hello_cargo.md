# Hello, Cargo

Cargo is Rusts build system and package manager. This tool can be used to manage Rust projects - since it handles alot of tasks such as building code, downloading libraries (code dependencies) and building libraries.

Cargo comes installed with Rust by default. We can check our install with the following snippet:

```bash
cargo --version
```

## TL;DR

- We can create a new project using `cargo new`
- We can build a project using `cargo build`
- We can build and run a project using `cargo run`
- We can build a project without building an executable using `cargo check`
- Instead of saving the result to the current directory, cargo stores it in the `target/debug` directory.
- Build an optimized executable with `cargo build --release`. The executable will be stored in the `target/release` directory.

## Creating a project

We can create a new project with the following command:

```bash
cargo new hello_cargo
```

We can then open the project folder and list the files:

```bash
cd hello_cargo
ls
```

Cargo has created two files for us:
- `Cargo.toml`: TOML is a configuration file format for our Cargo project. This file contains metadata about our project.
- `src/main.rs`: A hello world source code folder

# The Cargo.toml file

The cargo file should contain the following contents:
    
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- The first line `[package]` is a section heading that indicates that the following statements are configuring a package. 
- The next three lines set the configuration information Cargo needs to compile our program - The name version and the Rust edition to use.
- The last line `[dependecies]` is the start of a section where we can list the project dependencies. Packages of code are called *crates* in Rust.

If the cargo project isn't within a Git repository already - it also initializes a new Git repository for us.

## The main.rs file

The main.rs file should contain the following contents:

```rust
fn main() {
    println!("Hello, world!");
}
```

- The main difference between our original hello world project and the one generated with Cargo is that Cargo places the code in the `src` directory and we have a `Cargo.toml` configuration file.
- We can convert non-cargo projects into a Cargo project by moving the main file into a src directory and creating a `Cargo.toml` file.

## Building and running a cargo project

We can build our project with the following command:

```bash
cargo build
```

Thiscreates an executable file in the directory `target/debug/hello_cargo.exe`. We can run the executable with the following command:

```bash
.\target\debug\hello_cargo.exe
```

Also, after running Cargo build for the first time - Cargo creates a file called `Cargo.lock`. This file keeps track of the versions of the dependencies in our project. This file doesn't need to be modified manually - Cargo manages it for us.

We can also use `cargo run` to compile the code and run the resultant executable in one step (which is a lot more convenient):

```bash
cargo run
```

Notice that Cargo didn't rebuild the code. This is because it can detect that the code didn't change, so there was no need to recompile it.

The command `cargo check` can be used to check if the code compiles without actually building the executable.

```bash
cargo check
```

## Building for release

We can build our project for release with the following command:

```bash
cargo build --release
```

This command will create an executable in the `target/release` directory. This executable is optimized for speed and size - but takes longer to build. This is why we have two different profiles - we want speed to build when developing - and speed to execute when deploying.
