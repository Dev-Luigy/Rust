use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number.");
    println!("Input your guess: ");

    let number: i8 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Insert a number!");
                continue;
            }
        };

        if guess == number {
            println!("Correct!");
            break;
        } else {
            let message = format!("Too {}", if guess < number { "small" } else { "great" });
            println!("{message}");
            false
        };
    }
}
