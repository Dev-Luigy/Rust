use std::io;

fn read_number() -> i8 {
    loop {
        print!("Enter a number: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid integer number."),
        }
    }
}

fn main() {
    let mut numbers: [i8; 2] = [0; 2];

    for i in 0..numbers.len() {
        numbers[i] = read_number();
    }

    let [dividend, divisor] = numbers;

    if divisor == 0 {
        println!("Error: Division by zero is not allowed.");
    } else {
        println!(
            "Division {dividend} / {divisor} = {}\nRest = {}",
            dividend / divisor,
            dividend % divisor
        );
    }
}
