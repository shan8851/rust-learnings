use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // Generate a random number between 1 and 10
    let secret_number = rand::thread_rng().gen_range(1..=10);
    // Initiate user lives
    let mut lives = 3;

    loop {
        // print simple instructions for the user
        println!("Guess a number between 1 and 10. You have {lives} lives left.");
        // Create a mutable variable to store the user's guess
        let mut guess = String::new();
        // Read the user's input and store it in the guess variable
        io::stdin()
            .read_line(&mut guess)
            // Handle any errors that occur
            .expect("Failed to read line");
        // Convert the user's input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Print the user's guess
        println!("You guessed: {guess}");
        // Compare the user's guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                // If guess is lower than the secret number, notify the user and remove a life
                println!("Too small!");
                lives -= 1;
            },
            Ordering::Greater => {
                // If guess is higher than the secret number, notify the user and remove a life
                println!("Too big!");
                lives -= 1;
            },
            Ordering::Equal => {
                // If guess is equal to the secret number, notify the user and break the loop
                // exiting the program
                println!("You win!");
                break;
            }
        }

        if lives == 0 {
            // If the user has no lives left, notify the user and break the loop, exiting the program
            println!("You've run out of lives! The secret number was {secret_number}. Game over!");
            break;
        }
    }
}
