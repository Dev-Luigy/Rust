use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'birthdayCakeCandles' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY candles as parameter.
 */
fn birthday_cake_candles(candles: &[i32]) -> i32 {
    if candles.is_empty() {
        return 0;
    }
    let tallest: i32 = *candles.iter().max().unwrap();
    candles.iter().filter(|&&x| tallest == x).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let candles: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthday_cake_candles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}

