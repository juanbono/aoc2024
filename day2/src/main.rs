use day2::{count_safe_rows, parse_input};

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let input = parse_input(INPUT);
    let first_solution = count_safe_rows(input.clone(), false);
    println!("First solution: {first_solution}");

    let second_solution = count_safe_rows(input, true);

    println!("Second Solution: {second_solution}");
}
