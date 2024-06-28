### Rust Tutorials

Following along with the Rust Book at https://doc.rust-lang.org/book

# Chapter 1: Getting Started

## 1.1 Installation

-   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` to install Rust
-   rustup installs `rustc` (the Rust compiler), `cargo` (the package manager), and `rustup` (the toolchain manager)
-   `rustup update` to update Rust
-   `rustup self uninstall` to uninstall Rust

## 1.2 Hello, World!

-   `rustc main.rs` to compile a Rust program
-   `./main` to run the compiled program
-   `println!` is a macro that prints text to the console

## 1.3 Hello, Cargo!

-   `cargo new hello_cargo` creates a new project
-   `cargo build` builds the project
-   `cargo run` builds and runs the project
-   `cargo check` checks the project for errors without building it
-   `cargo build --release` builds the project in release mode

# Chapter 2: Programming a Guessing Game

-   `rand::Rng` is a trait that defines methods that random number generators implement
-   input dependencies in `Cargo.toml` under `[dependencies]` and run `cargo build` to download and compile them
-   expressions that return a Result can be handled with `match`
-   `std::cmp::Ordering` is an enum with variants `Less`, `Greater`, and `Equal`
-   `loop` creates an infinite loop
-   \_ is a catchall pattern that matches any value

# Chapter 3: Common Programming Concepts

## 3.1 Variables and Mutability

-   `let` is used to create a variable
-   `const` is used to create a constant
-   variables are immutable by default
-   `mut` is used to make a variable mutable
-   shadowing allows you to reuse a variable name

## 3.2 Data Types

-   Rust is statically typed
-   scalar types represent a single value
-   integer types include `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`, `isize`, and `usize`
-   floating-point types include `f32` and `f64`
-   `bool` is a boolean type, 1 byte in size
-   `char` is a Unicode scalar value (similar to runes in Golang)
-   compound types can group multiple values
-   tuples can hold multiple values of different types
-   tuples are defined like this: `(1, 2, 3)` and typed like this: `(i32, f64, u8)`
-   tuples are accessed with dot notation like this: `let x = (1, 2, 3); let y = x.0;`
-   arrays have a fixed length and all elements must be the same type
-   arrays are defined like this: `[1, 2, 3]` and typed like this: `[i32; 3]`
-   to initialize an array with the same value for each element, use the `array` macro like this: `[3; 5]` creates an array of 5 elements, all set to 3
-   arrays are accessed with square brackets like this: `let a = [1, 2, 3]; let b = a[0];`
-   if you try to access an array element that doesn't exist, Rust will panic with a runtime error
