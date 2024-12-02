#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
use std::collections::HashMap;

advent_of_code::solution!(1);

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    Some(
        left.into_iter()
            .zip(right)
            .filter_map(|(nl, nr)| u32::try_from((nl - nr).abs()).ok())
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    let mut occurrences = HashMap::new();
    for n in &left {
        occurrences.entry(*n).or_insert(0);
    }
    for n in right {
        occurrences
            .entry(n)
            .and_modify(|occurence| *occurence += 1)
            .or_insert(1);
    }

    Some(
        left.into_iter()
            .filter_map(|n| u32::try_from(n * occurrences[&n]).ok())
            .sum(),
    )
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
