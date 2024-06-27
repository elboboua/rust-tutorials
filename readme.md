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
