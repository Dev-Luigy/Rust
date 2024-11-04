use std::io;

fn read<T: std::str::FromStr>() -> Result<T, String> {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match input.parse::<T>() {
            Ok(num) => return Ok(num),
            Err(_) => {
                println!(
                    "Please insert a valid value of type {}",
                    std::any::type_name::<T>()
                );
            }
        }
    }
}

struct Product {
    quantity: i8,
    value: f32,
}

fn main() {
    let 
}
