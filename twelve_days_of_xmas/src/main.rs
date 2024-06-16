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
