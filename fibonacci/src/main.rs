use std::io;
use colored::*;

const MAX_FIBONACCI_INDEX: u64 = 93;

fn main() {
    println!("Enter the position of the Fibonacci sequence you want (max {}):", MAX_FIBONACCI_INDEX);

    let n: u64 = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) if num <= MAX_FIBONACCI_INDEX => break num,
            Ok(_) => println!("{}", format!("Please enter a valid number between 0 and {}.", MAX_FIBONACCI_INDEX).red()),
            Err(_) => println!("{}", format!("Invalid input. Please enter a number between 0 and {}.", MAX_FIBONACCI_INDEX).red()),
        }
    };

    println!("Do you want to see the entire sequence up to this number? (yes/no)");

    let show_sequence = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "yes" => break true,
            "no" => break false,
            _ => println!("{}", "Invalid input. Please enter 'yes' or 'no'.".red()),
        }
    };

    let result = fibonacci_iterative(n);

    println!("{}", format!("The {}th number in the Fibonacci sequence is {}", n, result).green());

    if show_sequence {
        println!("{}", format!("The Fibonacci sequence up to the {}th number is:", n).blue());
        let sequence = fibonacci_sequence(n);
        for num in sequence {
            print!("{} ", num.to_string().yellow());
        }
        println!();
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

fn fibonacci_sequence(n: u64) -> Vec<u64> {
    let mut sequence = Vec::new();
    let mut a = 0;
    let mut b = 1;

    for _i in 0..=n {
        sequence.push(a);
        let temp = a + b;
        a = b;
        b = temp;
    }

    sequence
}
