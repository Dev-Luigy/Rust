use std::io;

fn read<T: std::str::FromStr>() -> T {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match input.parse::<T>() {
            Ok(num) => return num,
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
    let mut products: Vec<Product> = Vec::new();

    for _ in 0..3 {
        let quantity: i8 = read();

        let product = Product {
            quantity,
            value: 0.0,
        };

        products.push(product);
    }

    for i in 0..3 {
        let value: f32 = read();

        products[i].value = value;
    }

    let mut client_money: f32 = read();

    for (_, product) in products.iter().enumerate() {
        client_money -= f32::from(product.quantity) * product.value
    }

    println!("{:.2}", client_money);
}
