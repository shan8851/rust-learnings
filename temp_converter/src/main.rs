use std::io;

fn main() {
    println!("Welcome to the temp convertor");
    println!("Enter the temperature in Fahrenheit");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");
    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };
    println!("You entered: {}°F", fahrenheit);
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("The temperature in Celsius is: {}°C", celsius);
}
