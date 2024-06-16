use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut lives = 3;

    loop {
        println!("Guess a number between 1 and 10. You have {lives} lives left.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                lives -= 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                lives -= 1;
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if lives == 0 {
            println!("You've run out of lives! The secret number was {secret_number}. Game over!");
            break;
        }
    }
}
