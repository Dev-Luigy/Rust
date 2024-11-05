use std::io;

fn read_integer() -> i8 {
    loop {
        let mut reader: String = String::new();

        io::stdin()
            .read_line(&mut reader)
            .expect("Error reading stdin!");

        match reader.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Insert a number of integer!");
                continue;
            }
        }
    }
}

fn main() {
    let mut numbers: [i8; 2] = [0; 2];

    println!("Insert the first number:");
    numbers[0] = read_integer();
    println!("Insert the second number:");
    numbers[1] = read_integer();

    println!("Greater is: ");
    println!("{}", numbers[0].max(numbers[1]));
}
