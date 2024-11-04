use std::io;

fn read_number() -> i8 {
    let mut input: String = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Can't reach input file.");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Must be a number!"),
        }
    }
}

fn main() {
    println!("Insert the first number: ");
    let n1 = read_number();
    println!("Insert the second number: ");
    let n2 = read_number();

    println!(
        "Average between numbers are: {:.1}", // This is lenght of numbers after dot.
        f32::from(n1 + n2) / 2 as f32 // You can't divide Float between integer, this is bizarre.
    );
}
