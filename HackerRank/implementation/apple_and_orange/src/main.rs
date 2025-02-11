use std::io::{self, BufRead};

/*
 * Complete the 'countApplesAndOranges' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER s
 *  2. INTEGER t
 *  3. INTEGER a
 *  4. INTEGER b
 *  5. INTEGER_ARRAY apples
 *  6. INTEGER_ARRAY oranges
 */
fn check_fruit_is_on_house(
    tree_position: i32,
    house_initial_point: i32,
    house_end_point: i32,
    fruit: i32,
) -> bool {
    let fruit_position = fruit + tree_position;
    house_initial_point <= fruit_position && house_end_point >= fruit_position
}

fn verify_fruit_quantity_per_tree(tree_position: i32, s: i32, t: i32, tree: &[i32]) -> i32 {
    tree.iter()
        .filter(|&&fruit| check_fruit_is_on_house(tree_position, s, t, fruit))
        .count() as i32
}

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apples_in_house: i32 = verify_fruit_quantity_per_tree(a, s, t, apples);
    let oranges_in_house: i32 = verify_fruit_quantity_per_tree(b, s, t, oranges);

    println!("{}", apples_in_house);
    println!("{}", oranges_in_house);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let third_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let m = third_multiple_input[0].trim().parse::<i32>().unwrap();

    let n = third_multiple_input[1].trim().parse::<i32>().unwrap();

    let apples: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}
