use std::io;

fn main() {
    let mut i: i8 = 0;
    let mut total: i8 = 0;

    while i < 3 {
        let mut reader = String::new();

        io::stdin()
            .read_line(&mut reader)
            .expect("Failed to read line");

        match reader.trim().parse::<i8>() {
            Ok(num) => {
                total += num;
                i += 1;
            }
            Err(_) => {
                println!("Insert a valid number.");
            }
        }
    }

    println!("Sum: {}", total);
}
