use std::io;

fn read() -> i8 {
    loop {
        let mut reader: String = String::new();

        io::stdin()
            .read_line(&mut reader)
            .expect("Error reading stdin!");

        match reader.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please insert an integer number.");
                continue;
            }
        }
    }
}

fn main() {
    let num = read();
    let num = if num % 2 != 0 { num - 1 } else { num };

    let mut even: Vec<i8> = Vec::new();

    for i in (2..=num).step_by(2) {
        even.push(i);
    }

    for i in (1..num).step_by(2) {
        println!("{}", i);
    }

    for i in (0..even.len()).rev() {
        println!("{}", even[i]);
    }
}
