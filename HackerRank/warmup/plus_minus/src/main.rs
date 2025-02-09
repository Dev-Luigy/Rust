use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let arr_len = arr.len() as f32;
    let positives: f32 = arr.iter().filter(|&&x| x.is_positive()).count() as f32;
    let negatives: f32 = arr.iter().filter(|&&x| x.is_negative()).count() as f32;
    let zeros: f32 = arr.iter().filter(|&&x| x == 0).count() as f32;

    let prositives_fraction: f32 = positives / arr_len;
    let negatives_fraction: f32 = negatives / arr_len;
    let zeroes_fraction: f32 = zeros / arr_len;

    println!("{:.6}", prositives_fraction);
    println!("{:.6}", negatives_fraction);
    println!("{:.6}", zeroes_fraction);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}
