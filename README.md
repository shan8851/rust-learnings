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

### Convert Temperature

A simple program that converts temperature from Fahrenheit to Celsius. The user will enter temp in f and be returned the correct temp in c.

The point of this is to familiarise myself with the `std::io` library and the `Result` type as well as work with data types and error handling in Rust.

### Fibonacci Sequence

A program that calculates the nth Fibonacci number and optionally displays the entire sequence up to that number. The user inputs a position in the Fibonacci sequence, and the program outputs the corresponding Fibonacci number. Additionally, the user can choose to see the entire sequence up to that number.

This program was created to practice working with loops, conditionals, and vector data structures in Rust. It also demonstrates basic error handling and user input validation.

*Key features and concepts covered:*

- Input Validation: Ensuring the user inputs a valid number within the allowed range (0 to 93).
- Iterative Calculation: Using a loop to calculate Fibonacci numbers efficiently.
- Optional Sequence Display: Allowing the user to choose whether to see just the nth number or the entire sequence up to that number.
- Colored Output: Utilizing the colored crate to enhance the user interface with colored text.


*This program uses the following Rust libraries and techniques:*

- std::io for reading user input
- std::vec::Vec for storing the Fibonacci sequence
- colored crate for adding color to terminal output

This project reinforces the use of basic Rust syntax and control flow, while also introducing external crates to enhance functionality and user experience.

### Twelve Days of Christmas

A Rust program that prints the lyrics to the Christmas carol "The Twelve Days of Christmas." The program takes advantage of the repetition in the song by using loops and arrays to generate the lyrics.

This program was created to practice using loops, arrays, and basic control flow in Rust. It demonstrates how to efficiently handle repetitive text generation through code.

*Key features and concepts covered:*

- Arrays for Static Data: Simple days array and also a gifts array in the corresponding order.
- Nested Loops for Repetition: An outer loop iterates over each day, and an inner loop iterates over the gifts for each day in reverse order to generate the correct lyrics.
- Conditional Logic: Uses a condition to add "and" before the first gift on days after the first day to match the song's structure.

Here is the code:

```rust
fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree.",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming"
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas", days[day]);
        println!("my true love sent to me:");

        for gift in (0..=day).rev() {
            if day > 0 && gift == 0 {
                print!("and ");
            }
            println!("{}", gifts[gift]);
        }
        println!();
    }
}
```

This program demonstrates handling repetitive tasks in rust and uses arrays, loops, string formatting and conditional logic to generate the lyrics to the song.
