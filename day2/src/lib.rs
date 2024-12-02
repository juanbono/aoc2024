use std::cmp::Ordering;

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut rows = vec![];

    for line in input.lines() {
        let row = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        rows.push(row);
    }
    rows
}

fn is_safe(row: Vec<u32>) -> bool {
    let mut sorted_increasing = row.to_owned();
    sorted_increasing.sort();
    let mut sorted_decreasing = row.to_owned();
    sorted_decreasing.sort_by(|a, b| b.cmp(a));

    let is_sorted = row == sorted_increasing || row == sorted_decreasing;

    if is_sorted {
        let mut levels_are_ok = false;

        let mut row_iter = row.into_iter();
        let mut current_elem = row_iter.next();

        while current_elem.is_some() {
            if let Some(next_elem) = row_iter.next() {
                let diff = current_elem.unwrap().abs_diff(next_elem);
                levels_are_ok = (1..=3).contains(&diff);

                if !levels_are_ok {
                    return false;
                }

                current_elem = Some(next_elem);
            } else {
                break;
            }
        }

        return levels_are_ok;
    }

    false
}

pub fn count_safe_rows(rows: Vec<Vec<u32>>, dampener: bool) -> usize {
    if dampener {
        rows.into_iter()
            .filter(|row| is_safe_with_dampener(row.to_vec()))
            .count()
    } else {
        rows.into_iter().filter(|row| is_safe(row.to_vec())).count()
    }
}

fn is_safe_with_dampener(row: Vec<u32>) -> bool {
    let mut dampener_used = false;
    let mut sorted_increasing = row.to_owned();
    sorted_increasing.sort_by(|a, b| match a.cmp(b) {
        o @ (Ordering::Less | Ordering::Equal) => {
            if !dampener_used {
                dampener_used = true;
                Ordering::Greater
            } else {
                o
            }
        }
        o => o,
    });
    let mut sorted_decreasing = row.to_owned();
    sorted_decreasing.sort_by(|a, b| match b.cmp(a) {
        o @ (Ordering::Greater | Ordering::Equal) => {
            if !dampener_used {
                dampener_used = true;
                Ordering::Less
            } else {
                o
            }
        }
        o => o,
    });

    let is_sorted = row == sorted_increasing || row == sorted_decreasing;

    let mut fix_level = dampener_used;
    if is_sorted {
        let mut levels_are_ok = false;

        let mut row_iter = row.into_iter();
        let mut current_elem = row_iter.next();

        while current_elem.is_some() {
            if let Some(next_elem) = row_iter.next() {
                let diff = current_elem.unwrap().abs_diff(next_elem);
                levels_are_ok = (1..=3).contains(&diff);

                match (!levels_are_ok, fix_level) {
                    (true, true) => {
                        fix_level = false;
                    }
                    (true, false) => {
                        return false;
                    }
                    _ => {}
                }

                current_elem = Some(next_elem);
            } else {
                break;
            }
        }

        return levels_are_ok;
    }

    false
}

#[test]
fn test_is_safe() {
    let row1 = vec![7, 6, 4, 2, 1];

    assert!(is_safe(row1));

    let row2 = vec![1, 2, 7, 8, 9];

    assert!(!is_safe(row2));
}

#[test]
fn test_is_safe_with_dampener() {
    let row = vec![7, 6, 4, 2, 1];
    // should be safe without using the dampener
    //assert!(is_safe_with_dampener(row));

    let row_should_use_dampener = vec![1, 3, 2, 4, 5];
    //assert!(is_safe_with_dampener(row_should_use_dampener));

    let row_with_dampener2 = vec![8, 6, 4, 4, 1];
    assert!(is_safe_with_dampener(row_with_dampener2));
}
