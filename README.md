# Rust Documentation
It is a very low level programming language (system level programming language) like C,C++. It focuses on safety , speed, and concurrency. And it has a design that lets you create programs that have high performance but it also has a huge level of control and has the power of some of the high level language abstractions that we don't get in any other low level languages like C++ and C.Rust doesn't have garbage collection instead it uses something called Ownership.

## Installation & First Project
To install rust on your computer just go to https://www.rust-lang.org/ . 
And then install the Rustup program which is the tool that allows you to manage multiple versions of Rust on your computer, stable build and development build. All three builds gets installed on your computer at once without having any issues. With RustUp you get the rust compiler which is called Rust C and you also gain access to the Rust package manager which is called cargo.To check Rustup in cargo, you can just type `rustup` to view all the different type of commands and you can view the various types of toolchain that is installed in the machine. And the `cargo` command shows a bunch of cargo commands.In Rust, there are certain conventions that need to be followed when we are creating dependencies. There are dependencies on libraries in Rust. Cargo! is a dependency manager in Rust that is similar to npm (in JavaScript) and pip (in Python). All the project dependencies are handled by cargo.

To start a new project you need to create a project directory to store all the project programs. To create a new rust project type this code in terminal
```sh
cargo new hello_world
```
This code will basically create a new directory as hello_world with an src file which contains the main.rs file.
```sh
cargo new hello_world -- bin
```
This code will basically create a binary application hello_world project.
In the created directory there will be a cargo.toml file which is the package file.

