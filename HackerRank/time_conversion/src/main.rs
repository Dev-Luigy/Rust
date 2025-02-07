use core::time;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn time_conversion(s: &str) -> String {
    let period = &s[s.len() - 2..];
    let time_parts: Vec<&str> = s[..s.len() - 2].split(':').collect();

    let mut hour: i32 = time_parts[0].parse().expect("Hora inválida");
    let minute: i32 = time_parts[1].parse().expect("Minuto inválido");
    let second: i32 = time_parts[2].parse().expect("Segundo inválido");

    if period == "PM" && hour != 12 {
        hour += 12;
    } else if period == "AM" && hour == 12 {
        hour = 0;
    }

    format!("{:02}:{:02}:{:02}", hour, minute, second)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}

