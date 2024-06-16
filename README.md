# Rust Learning

A simple repo to house and dump all my rust learnings and mini projects.

### Hello World

A simple hello world program to get started with rust. Simply print `Hello, World!` to the console.

### Hello Cargo

A simple hello world program using cargo to get started with rust. Simply print `Hello, Cargo!` to the console.

### Guessing Game

Building on the example from the [rust book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) this is a simple program that generates a random number and asks the user to guess it. The user can input a number and the program will tell the user if the number is too high or too low.

To build on the example I made the secret number between 1 and 10, gave the user only 3 lives and added a message to the user if they run out of lives.

This program was built to start using different utilities from the rust standard library:

- `io::stdin()` to read user input
- `rand::Rng` to generate random numbers
- `cmp::Ordering` to compare numbers and flesh out the game logic

Both mutable and immutable variables were used to keep track of the game state and the user input.
