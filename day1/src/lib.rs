use std::{collections::HashMap, error::Error};

pub fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let mut left_numbers = vec![];
    let mut right_numbers = vec![];

    for line in input.lines() {
        let mut numbers_in_line = line.split_whitespace().map(|h| h.trim().parse::<u32>());
        left_numbers.push(numbers_in_line.next().expect("malformed file")?);
        right_numbers.push(numbers_in_line.next().expect("malformed file")?);
    }

    Ok((left_numbers, right_numbers))
}

pub fn calculate_total_distance(left_numbers: &[u32], right_numbers: &[u32]) -> u32 {
    let mut total = 0;
    let mut sorted_left = left_numbers.to_owned();
    let mut sorted_right = right_numbers.to_owned();
    sorted_left.sort();
    sorted_right.sort();

    for (l, r) in sorted_left.into_iter().zip(sorted_right) {
        total += l.abs_diff(r)
    }

    total
}

pub fn calculate_total_similarity(left_numbers: &[u32], right_numbers: &[u32]) -> u32 {
    let mut total = 0;
    let mut similarities = HashMap::new();

    for l in left_numbers {
        similarities.insert(l, 0);
    }

    for r in right_numbers {
        if similarities.contains_key(r) {
            *similarities.entry(r).or_default() += 1;
        }
    }

    for (l, q) in similarities {
        total += l * q;
    }

    total
}
