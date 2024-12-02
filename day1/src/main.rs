use std::error::Error;

pub use day1::{calculate_total_distance, calculate_total_similarity, parse_input};

static INPUT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let (left_numbers, right_numbers) = parse_input(INPUT)?;
    let first_solution = calculate_total_distance(&left_numbers, &right_numbers);

    println!("First solution: {first_solution}");

    let second_solution = calculate_total_similarity(&left_numbers, &right_numbers);

    println!("Second solution: {second_solution}");

    Ok(())
}
