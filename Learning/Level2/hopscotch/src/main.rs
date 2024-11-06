use std::io;

fn read<T: std::str::FromStr>() -> T {
    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading stdin!");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Insert a valid {}.", std::any::type_name::<T>());
                continue;
            }
        }
    }
}

fn main() {
    let stone: i8 = read();
    let mut foot: String = read();

    print!("[");
    for i in 0..=10 {
        if i == stone || stone == 10 {
            continue;
        };

        print!(" {}{}", i, foot);
        foot = if foot == "d" {
            "e".to_string()
        } else {
            "d".to_string()
        };
    }
    print!(" ]");
}
