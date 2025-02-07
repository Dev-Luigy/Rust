/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i64]) {
    let min_sum: i64 = arr.iter().sum::<i64>() - *arr.iter().min().expect("Array is empty");
    let max_sum: i64 = arr.iter().sum::<i64>() - *arr.iter().max().expect("Array is empty");

    println!("{} {}", max_sum, min_sum);
}

fn main() {
    let arr: Vec<i64> = vec![156873294, 719583602, 581240736, 605827319, 895647130];

    mini_max_sum(&arr);
}
